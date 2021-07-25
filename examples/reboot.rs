use daemon_ctrl::Contral;

const CTRL_FILE: &str = "a/a/a/a";

fn main() {
    let mut ctrl = Contral::read(CTRL_FILE);
    println!("now: {:?}", ctrl);
    ctrl.reboot = true;
    ctrl.save(CTRL_FILE);
}
