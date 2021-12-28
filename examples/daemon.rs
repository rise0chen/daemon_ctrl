use core::time::Duration;
use daemon_ctrl::{ctrl, WatchConfig};

const CTRL_FILE: &str = "a/a/a/a";

fn main() {
    simple_logger::SimpleLogger::new().init().unwrap();

    let mut cfg = WatchConfig::new();
    cfg.auto_restart(true);
    cfg.set_ctrl_file(String::from(CTRL_FILE)).unwrap();

    if let Ok(is_parent) = ctrl(cfg) {
        if is_parent {
            println!("into daemon.");
            std::process::exit(0);
        }
    } else {
        println!("not support");
    }

    // your program
    println!("running...");
    std::thread::sleep(Duration::from_secs(30));
    println!("exit");
}
