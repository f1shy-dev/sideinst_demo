#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusty_libimobiledevice::{
    idevice::{self, Device},
    services::userpref,
};

// get device name
#[tauri::command]
fn get_devices() -> String {
    let devices = match idevice::get_devices() {
        Ok(devices) => devices,
        Err(e) => {
            // If the daemon is not running or does not behave as expected, this returns an error
            return format!("Error getting devices: {:?}", e);
        }
    };
    let mut mp = Vec::with_capacity(devices.len());
    for device in devices {
        let lock_cli = match device.new_lockdownd_client("ss_downloader") {
            Ok(l) => l,
            Err(_) => continue,
        };
        let name = match lock_cli.get_device_name() {
            Ok(n) => n,
            Err(_) => continue,
        };
        // I hate this, but I'm lazy
        mp.push((
            device.clone(),
            format!(
                "{} ({})",
                name,
                if device.get_network() {
                    "Network"
                } else {
                    "USB"
                }
            ),
        ));
    }
    return format!("Got device info: {:?}", mp);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
