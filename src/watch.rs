use super::Contral;
use crate::consts::*;
use core::time::Duration;
use std::io::Result as IoResult;
use std::path::Path;
use std::process::{Child, Command};
use std::time::SystemTime;
use std::{fs, thread};

pub struct WatchConfig {
    ctrl_file: Option<(String, SystemTime)>,
    auto_restart: bool,
}
impl WatchConfig {
    pub fn new() -> Self {
        Self {
            ctrl_file: None,
            auto_restart: false,
        }
    }
    pub fn set_ctrl_file(&mut self, file: String) -> IoResult<()> {
        let path = Path::new(&file);
        if !path.exists() {
            fs::create_dir_all(path.parent().unwrap())?;
            fs::File::create(path)?;
        }
        let modtim = fs::metadata(path)?.modified().unwrap();
        self.ctrl_file = Some((file, modtim));
        Ok(())
    }
    pub fn auto_restart(&mut self, enable: bool) {
        self.auto_restart = enable;
    }
}

pub struct Watch {
    program: String,
    args: Vec<String>,
    cfg: WatchConfig,
    child: Option<Child>,
}
impl Watch {
    pub fn new(program: String, args: Vec<String>, cfg: WatchConfig) -> Self {
        Self {
            program,
            args,
            cfg,
            child: None,
        }
    }
    pub fn watch(&mut self) {
        loop {
            let ctrl_changed = {
                if let Some((file, modtim)) = &mut self.cfg.ctrl_file {
                    let attr = fs::metadata(&*file).unwrap();
                    let modtim_now = attr.modified().unwrap();
                    if &modtim_now != modtim {
                        *modtim = modtim_now;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            if ctrl_changed {
                log::info!("ctrl file changed.");
                let file = self.cfg.ctrl_file.as_ref().unwrap().0.clone();
                let mut ctrl = Contral::read(&file);

                let mut need_save = false;
                if ctrl.reboot {
                    ctrl.reboot = false;
                    self.child_restart();
                    need_save = true;
                }
                if need_save {
                    ctrl.save(&file);
                }
            }

            if self.cfg.auto_restart {
                let need_start = {
                    let mut ret = true;
                    if let Some(child) = &mut self.child {
                        if let Ok(result) = child.try_wait() {
                            if let None = result {
                                ret = false;
                            }
                        }
                    }
                    ret
                };
                if need_start {
                    self.child_restart();
                }
            }
            thread::sleep(Duration::from_millis(1000));
        }
    }
    fn child_restart(&mut self) {
        log::info!("restart {}:{:?}", self.program, self.args);
        if let Some(child) = &mut self.child {
            let _err = child.kill();
        }
        let child = Command::new(&self.program)
            .args(&self.args)
            .env(CHILD_ENV_KEY, CHILD_ENV_VAL)
            .spawn()
            .unwrap();
        self.child = Some(child);
    }
}
