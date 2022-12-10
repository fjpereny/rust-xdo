
use std::process::Command;


pub(crate) fn press(key: &str) {
        Command::new("xdotool")
        .arg("key")
        .arg(key)
        .spawn()
        .expect("Command failed to spawn");
}