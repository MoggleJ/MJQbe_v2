//! CPU / memory / disk / network / temperature, from `/proc` + `sysfs`.

use std::time::Duration;

use crate::domain::{CoreError, SystemSnapshot};

/// Take a resource snapshot. CPU % and network rates are sampled over `window`
/// (a second `/proc` read after a short sleep).
pub async fn snapshot() -> Result<SystemSnapshot, CoreError> {
    let window = Duration::from_millis(200);

    let cpu1 = read_cpu_times()?;
    let net1 = read_net_totals()?;
    tokio::time::sleep(window).await;
    let cpu2 = read_cpu_times()?;
    let net2 = read_net_totals()?;

    let cpu_percent = cpu_delta_percent(cpu1, cpu2);
    let secs = window.as_secs_f64();
    let net_rx_bytes_per_s = ((net2.0.saturating_sub(net1.0)) as f64 / secs) as u64;
    let net_tx_bytes_per_s = ((net2.1.saturating_sub(net1.1)) as f64 / secs) as u64;

    let (mem_total_kb, mem_used_kb, swap_total_kb, swap_used_kb) = read_meminfo()?;
    let (disk_total_kb, disk_used_kb) = read_disk("/")?;

    Ok(SystemSnapshot {
        cpu_percent,
        load_avg: read_loadavg().unwrap_or([0.0; 3]),
        mem_total_kb,
        mem_used_kb,
        swap_total_kb,
        swap_used_kb,
        disk_total_kb,
        disk_used_kb,
        net_rx_bytes_per_s,
        net_tx_bytes_per_s,
        temp_celsius: read_temp_celsius(),
        uptime_secs: read_uptime().unwrap_or(0),
    })
}

fn proc_read(path: &str) -> Result<String, CoreError> {
    std::fs::read_to_string(path).map_err(|e| CoreError::Internal(format!("read {path}: {e}")))
}

/// (idle+iowait, total) jiffies from the aggregate `cpu` line of /proc/stat.
fn read_cpu_times() -> Result<(u64, u64), CoreError> {
    let stat = proc_read("/proc/stat")?;
    let line = stat
        .lines()
        .next()
        .filter(|l| l.starts_with("cpu "))
        .ok_or_else(|| CoreError::Internal("no cpu line in /proc/stat".into()))?;
    let vals: Vec<u64> = line
        .split_whitespace()
        .skip(1)
        .filter_map(|v| v.parse().ok())
        .collect();
    if vals.len() < 5 {
        return Err(CoreError::Internal("short cpu line".into()));
    }
    let idle = vals[3] + vals[4]; // idle + iowait
    let total: u64 = vals.iter().sum();
    Ok((idle, total))
}

pub(crate) fn cpu_delta_percent(a: (u64, u64), b: (u64, u64)) -> f64 {
    let idle = b.0.saturating_sub(a.0) as f64;
    let total = b.1.saturating_sub(a.1) as f64;
    if total <= 0.0 {
        0.0
    } else {
        (100.0 * (1.0 - idle / total)).clamp(0.0, 100.0)
    }
}

/// Aggregate (rx_bytes, tx_bytes) over every interface except loopback.
fn read_net_totals() -> Result<(u64, u64), CoreError> {
    let dev = proc_read("/proc/net/dev")?;
    let mut rx = 0u64;
    let mut tx = 0u64;
    for line in dev.lines().skip(2) {
        let Some((name, rest)) = line.split_once(':') else {
            continue;
        };
        if name.trim() == "lo" {
            continue;
        }
        let cols: Vec<u64> = rest
            .split_whitespace()
            .filter_map(|v| v.parse().ok())
            .collect();
        if cols.len() >= 9 {
            rx += cols[0];
            tx += cols[8];
        }
    }
    Ok((rx, tx))
}

/// (mem_total, mem_used, swap_total, swap_used) in kB.
fn read_meminfo() -> Result<(u64, u64, u64, u64), CoreError> {
    let info = proc_read("/proc/meminfo")?;
    let get = |key: &str| -> u64 {
        info.lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split_whitespace().nth(1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    };
    let mem_total = get("MemTotal:");
    let mem_avail = get("MemAvailable:");
    let swap_total = get("SwapTotal:");
    let swap_free = get("SwapFree:");
    Ok((
        mem_total,
        mem_total.saturating_sub(mem_avail),
        swap_total,
        swap_total.saturating_sub(swap_free),
    ))
}

fn read_disk(path: &str) -> Result<(u64, u64), CoreError> {
    let c_path = std::ffi::CString::new(path).unwrap();
    // SAFETY: statvfs writes into a zeroed struct we own; path is a valid CString.
    let mut vfs: libc::statvfs = unsafe { std::mem::zeroed() };
    let rc = unsafe { libc::statvfs(c_path.as_ptr(), &mut vfs) };
    if rc != 0 {
        return Err(CoreError::Internal(format!("statvfs({path}) failed")));
    }
    let block = vfs.f_frsize.max(1) as u64;
    let total_kb = vfs.f_blocks as u64 * block / 1024;
    let avail_kb = vfs.f_bavail as u64 * block / 1024;
    Ok((total_kb, total_kb.saturating_sub(avail_kb)))
}

fn read_loadavg() -> Option<[f64; 3]> {
    let s = std::fs::read_to_string("/proc/loadavg").ok()?;
    let mut it = s.split_whitespace();
    Some([
        it.next()?.parse().ok()?,
        it.next()?.parse().ok()?,
        it.next()?.parse().ok()?,
    ])
}

fn read_uptime() -> Option<u64> {
    let s = std::fs::read_to_string("/proc/uptime").ok()?;
    s.split_whitespace()
        .next()?
        .parse::<f64>()
        .ok()
        .map(|v| v as u64)
}

/// First readable thermal zone, in °C. Pi exposes `/sys/class/thermal/thermal_zone0`.
fn read_temp_celsius() -> Option<f64> {
    for zone in 0..8 {
        let path = format!("/sys/class/thermal/thermal_zone{zone}/temp");
        if let Ok(raw) = std::fs::read_to_string(&path) {
            if let Ok(milli) = raw.trim().parse::<f64>() {
                return Some(milli / 1000.0);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_percent_is_bounded_and_sane() {
        // 50% busy: idle advanced 50, total advanced 100.
        assert!((cpu_delta_percent((100, 1000), (150, 1100)) - 50.0).abs() < 1e-9);
        // No movement → 0.
        assert_eq!(cpu_delta_percent((1, 2), (1, 2)), 0.0);
        // Fully busy.
        assert_eq!(cpu_delta_percent((10, 100), (10, 200)), 100.0);
    }

    #[tokio::test]
    async fn snapshot_reads_this_host() {
        // Runs on any Linux box (CI included).
        let s = snapshot().await.unwrap();
        assert!(s.mem_total_kb > 0);
        assert!(s.disk_total_kb > 0);
        assert!(s.cpu_percent >= 0.0 && s.cpu_percent <= 100.0);
    }
}
