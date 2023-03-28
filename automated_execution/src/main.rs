use std::process::{Command, Stdio};

fn main() {
    println!("Checking on device status...");
    if !check_device() {
        if !launch_device() {
            std::process::exit(1);
        }
    }

}

fn check_device() -> bool {
    let adb = Command::new("adb").arg("devices").stdout(Stdio::piped())
                                .output().expect("adb tool is not installed");
    let adb_output = String::from_utf8_lossy(&adb.stdout).to_string();

    if !adb_output.contains("emulator") {
        false
    } else {
        true
    }
}

fn launch_device() -> bool {
    let emulator = Command::new("emulator").arg("-list-avds").stdout(Stdio::piped())
                                  .output().expect("\'emulator\' was not found.");
    let emulator_output = String::from_utf8_lossy(&emulator.stdout).to_string();

    if emulator_output.is_empty() {
        println!("An AVD image must be created first.\n");
        false
    } else {
        Command::new("emulator").arg("-avd").arg(emulator_output);
        true
    }
}