mod consts;
mod control;
mod watch;

use consts::*;
pub use control::Control;
use std::env;
use watch::Watch;
pub use watch::WatchConfig;

/// return: is_parent:bool
#[cfg(windows)]
pub fn ctrl(_cfg: WatchConfig) -> Result<bool, ()> {
    return Err(());
}

/// return: is_parent:bool
#[cfg(not(windows))]
pub fn ctrl(cfg: WatchConfig) -> Result<bool, ()> {
    let mut args = env::args();
    let program = args.next().unwrap();
    let args: Vec<String> = args.collect();
    if let Ok(ref val) = env::var(CHILD_ENV_KEY) {
        if val == CHILD_ENV_VAL {
            return Ok(false);
        }
    }
    use fork::{daemon, Fork};
    if let Ok(Fork::Child) = daemon(true, true) {
        let mut watch = Watch::new(program, args, cfg);
        watch.watch();
    }
    return Ok(true);
}
