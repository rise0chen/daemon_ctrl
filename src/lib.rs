mod contral;
mod watch;

pub use contral::Contral;
use fork::{daemon, Fork};
use watch::Watch;
pub use watch::WatchConfig;

/// return: is_parent:bool
pub fn ctrl(cfg: WatchConfig) -> bool {
    let mut args = std::env::args();
    let program = args.next().unwrap();
    let mut args: Vec<String> = args.collect();
    if let Some(_) = args.iter().find(|v| *v == "by_auto_restart") {
        return false;
    }

    if let Ok(Fork::Child) = daemon(true, true) {
        args.push(String::from("by_auto_restart"));
        let mut watch = Watch::new(program, args, cfg);
        watch.watch();
    }
    return true;
}
