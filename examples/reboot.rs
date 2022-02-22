use daemon_ctrl::Control;

const CTRL_FILE: &str = "a/a/a/a";

fn main() {
    let mut ctrl = Control::read(CTRL_FILE);
    println!("now: {:?}", ctrl);
    ctrl.reboot = true;
    ctrl.save(CTRL_FILE);
}
