# Daemon Ctrl

- auto restart
- request restart by file system

## Usage

### main

```rust
use core::time::Duration;
use daemon_ctrl::{ctrl, WatchConfig};

const CTRL_FILE: &str = "a/a/a/a";

fn main() {
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
}
```

### request restart by file system

```rust
use daemon_ctrl::Control;

const CTRL_FILE: &str = "a/a/a/a";

fn main() {
    let mut ctrl = Control::read(CTRL_FILE);
    println!("now: {:?}", ctrl);
    ctrl.reboot = true;
    ctrl.save(CTRL_FILE);
}
```
