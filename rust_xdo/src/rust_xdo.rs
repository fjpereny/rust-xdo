
use std::process::Command;

// KEYBOARD INPUTS

// KEYPRESS
pub(crate) fn press(key: &str) {
    Command::new("xdotool")
        .arg("key")
        .arg(key)
        .spawn()
        .expect("Command failed to spawn");
}

// TYPEWRITE
pub(crate) fn typewrite(text: String, delay: u32) {

    let delay = delay.to_string();

    Command::new("xdotool")
        .arg("type")
        .arg("--delay")
        .arg(delay)
        .arg(text)
        .spawn()
        .expect("Command failed to spawn");
}


// WINDOWS

// GET NAME OF ACTIVE WINDOW
pub(crate) fn get_active_window() -> &str {
    Command::new("xdotool")
        .arg("getactivewindow")
        .spawn()
        .expect("Command failed to spawn");
}