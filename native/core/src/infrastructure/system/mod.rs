//! Host monitoring for the native Dev mode — reads `/proc` and `sysfs`.
//! Everything here is Linux-only and works the same on the Pi and on a dev box.

mod docker;
mod metrics;
mod processes;

pub use docker::DockerCli;
pub use metrics::snapshot;
pub use processes::{list_processes, renice, terminate};
