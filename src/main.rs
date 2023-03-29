use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::fs;

// GLOBAL VARIABLES
const FILES_PATH: &str = "storage/self/primary/Download/";

fn main() {
    println!("Checking on device status...");
    if !check_device() {
        let emu_name = launch_device();
        if emu_name.len() == 0 {
            println!("Some error occurred trying to start a device.");
            std::process::exit(1);
        } else {
            println!("Device started successfully.");
        }
        install_apk(emu_name);
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

fn launch_device() -> String {
    let emulator = Command::new("emulator").arg("-list-avds").stdout(Stdio::piped())
                                  .output().expect("\'emulator\' was not found.");
    let emulator_output = String::from_utf8_lossy(&emulator.stdout).to_string();
    let mut avd = String::new();
    if emulator_output.is_empty() {
        println!("An AVD image must be created first."); // Cannot resolve to create avds with avdmanager: Batch script of Windows and .sh for Linux.
        avd
    } else {
        let avd_name = format!("@{}",emulator_output.trim());
        avd = avd_name.clone();
        let handle_launch = thread::spawn(|| { 
            Command::new("emulator").arg(avd_name).spawn().unwrap();
            thread::sleep(Duration::from_secs(5)); // Give 5 seconds to launch device
        });
        handle_launch.join().unwrap();
        avd
    }
}

fn add_files() {
    // Keep it simple. Files will be pushed to ../Download/.
    let files =  fs::read_dir("./files/").unwrap();

    for file in files {
        let push_file = file.unwrap().path().display().to_string();
        Command::new("adb").arg("push").arg(push_file).arg(FILES_PATH).spawn().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn install_apk(avd_name: String) {
    let paths = fs::read_dir("./packages/").unwrap();
    let handle_boot = thread::spawn(|| {
        Command::new("adb").arg("root").spawn().unwrap(); // Start as root
        thread::sleep(Duration::from_secs(15));
    });
    handle_boot.join().unwrap();

    for path in paths {
        let apk = path.unwrap().path().display().to_string();
        println!("Package {} to be installed.", apk);
        let handle_install = thread::spawn(|| { 
            Command::new("adb").arg("install").arg(apk).spawn().unwrap();
            thread::sleep(Duration::from_secs(5)); // Give 5 seconds for app to be installed.
        });
        handle_install.join().unwrap();
        start_apk_execution(avd_name.clone());
    }
}

fn start_apk_execution(avd_name: String) {
    add_files();

    // Clean data
    // To be fixed.
    let handle_clear = thread::spawn(|| { 
        Command::new("emulator").arg(avd_name).arg("-wipe-data").spawn().unwrap();
        thread::sleep(Duration::from_secs(5)); // Give 5 seconds for device to be cleared
    });
    handle_clear.join().unwrap();
}