mod contral;
mod watch;

pub use contral::Contral;
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
    let mut args = std::env::args();
    let program = args.next().unwrap();
    let mut args: Vec<String> = args.collect();
    if let Some(_) = args.iter().find(|v| *v == "by_auto_restart") {
        return Ok(false);
    }
    use fork::{daemon, Fork};
    if let Ok(Fork::Child) = daemon(true, true) {
        args.push(String::from("by_auto_restart"));
        let mut watch = Watch::new(program, args, cfg);
        watch.watch();
    }
    return Ok(true);
}
