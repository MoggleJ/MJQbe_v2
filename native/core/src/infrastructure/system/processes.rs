//! Process list + control, from `/proc/<pid>` and `libc`.

use crate::domain::{CoreError, ProcessInfo};

/// Every readable process, sorted by RSS descending, capped at `limit`.
pub fn list_processes(limit: usize) -> Result<Vec<ProcessInfo>, CoreError> {
    let mut out = Vec::new();

    let entries =
        std::fs::read_dir("/proc").map_err(|e| CoreError::Internal(format!("read /proc: {e}")))?;

    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(pid) = name.to_str().and_then(|s| s.parse::<i32>().ok()) else {
            continue;
        };
        if let Some(info) = read_one(pid) {
            out.push(info);
        }
    }

    out.sort_by_key(|p| std::cmp::Reverse(p.mem_rss_kb));
    out.truncate(limit.clamp(1, 500));
    Ok(out)
}

fn read_one(pid: i32) -> Option<ProcessInfo> {
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    let field = |key: &str| -> Option<String> {
        status
            .lines()
            .find(|l| l.starts_with(key))
            .map(|l| l[key.len()..].trim().to_string())
    };

    let name = field("Name:").unwrap_or_default();
    let state = field("State:")
        .map(|s| s.chars().next().map(String::from).unwrap_or(s))
        .unwrap_or_default();
    let mem_rss_kb = field("VmRSS:")
        .and_then(|v| v.split_whitespace().next().and_then(|n| n.parse().ok()))
        .unwrap_or(0);
    let threads = field("Threads:").and_then(|v| v.parse().ok()).unwrap_or(0);

    // niceness: field 19 of /proc/<pid>/stat (after the possibly-parenthesised comm).
    let nice = std::fs::read_to_string(format!("/proc/{pid}/stat"))
        .ok()
        .and_then(|stat| {
            let close = stat.rfind(')')?;
            let rest: Vec<&str> = stat[close + 2..].split_whitespace().collect();
            rest.get(16).and_then(|v| v.parse().ok()) // 19th field overall
        })
        .unwrap_or(0);

    Some(ProcessInfo {
        pid,
        name,
        state,
        mem_rss_kb,
        nice,
        threads,
    })
}

/// Send a signal (default `SIGTERM`) to a process.
pub fn terminate(pid: i32, signal: Option<i32>) -> Result<(), CoreError> {
    if pid <= 1 {
        return Err(CoreError::PermissionDenied(
            "refusing to signal pid <= 1".into(),
        ));
    }
    let sig = signal.unwrap_or(libc::SIGTERM);
    // SAFETY: plain libc call; errors surfaced via errno below.
    let rc = unsafe { libc::kill(pid, sig) };
    if rc != 0 {
        return Err(map_errno(&format!("kill({pid})")));
    }
    Ok(())
}

/// Set the niceness of a process (`-20`..`19`).
pub fn renice(pid: i32, niceness: i32) -> Result<(), CoreError> {
    if pid <= 1 {
        return Err(CoreError::PermissionDenied(
            "refusing to renice pid <= 1".into(),
        ));
    }
    let n = niceness.clamp(-20, 19);
    // SAFETY: plain libc call.
    let rc = unsafe { libc::setpriority(libc::PRIO_PROCESS, pid as libc::id_t, n) };
    if rc != 0 {
        return Err(map_errno(&format!("setpriority({pid})")));
    }
    Ok(())
}

fn map_errno(ctx: &str) -> CoreError {
    let err = std::io::Error::last_os_error();
    match err.raw_os_error() {
        Some(libc::EPERM) | Some(libc::EACCES) => {
            CoreError::PermissionDenied(format!("{ctx}: {err}"))
        }
        Some(libc::ESRCH) => CoreError::NotFound,
        _ => CoreError::Internal(format!("{ctx}: {err}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_at_least_this_process() {
        let procs = list_processes(50).unwrap();
        assert!(!procs.is_empty());
        assert!(procs.len() <= 50);
        let me = std::process::id() as i32;
        assert!(procs.iter().any(|p| p.pid == me) || procs.len() == 50);
    }

    #[test]
    fn refuses_to_signal_init() {
        assert!(matches!(
            terminate(1, None).unwrap_err(),
            CoreError::PermissionDenied(_)
        ));
        assert!(matches!(
            renice(1, 5).unwrap_err(),
            CoreError::PermissionDenied(_)
        ));
    }

    #[test]
    fn signalling_a_dead_pid_is_not_found() {
        // 0x3FFF_FFFF is far above any real pid.
        assert!(matches!(
            terminate(0x3FFF_FFFF, Some(0)).unwrap_err(),
            CoreError::NotFound
        ));
    }
}
