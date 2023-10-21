use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use std::process::Command;

pub fn new_window(s: &mut Cursive) {
    match Command::new("tmux").arg("new-window").output() {
        Ok(result) => {
            if result.stderr.len() != 0 {
                s.add_layer(
                    Dialog::around(TextView::new("Please Start Tmux :)")).button("exit", |s| s.quit()),
                    );
            }
        }
        Err(_) => {
            s.add_layer(
                Dialog::around(TextView::new("Please Start Tmxu :)")).button("exit", |s| s.quit()),
                );
        }
    };
}
pub fn directory(s: &mut Cursive, path: &str) {
    match Command::new("tmux")
        .arg("new-window")
        .arg("-c")
        .arg(path)
        .output()
    {
        Ok(result) => {
            if result.stderr.len() != 0 {
                s.add_layer(
                    Dialog::around(TextView::new("Please start Tmux :)"))
                        .button("exit", |s| s.quit()),
                );
            }
        }
        Err(_) => {
            s.add_layer(
                Dialog::around(TextView::new("Please start Tmux :)")).button("exit", |s| s.quit()),
            );
        }
    };
}

pub fn executable(s: &mut Cursive, path: &str) {
    let mut sh_command = String::from("/bin/sh -c ");
    sh_command.push_str(path);

    match Command::new("tmux")
        .arg("new-window")
        .arg(sh_command)
        .output()
    {
        Ok(result) => {
            if result.stderr.len() != 0 {
                s.add_layer(
                    Dialog::around(TextView::new("Please start Tmux :)")).button("exit", |s| s.quit()),
                );
            }
        }
        Err(_) => {
            s.add_layer(Dialog::around(TextView::new("Please start Tmux :)")).button("exit", |s| s.quit()));
        }
    };
}
