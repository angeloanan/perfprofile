mod profile;
mod tray;

use notify::{RecursiveMode, Watcher};
use notify_rust::Timeout;
use std::{fs, path::Path, thread};

const PLATFORM_PROFILE_PATH: &str = "/sys/firmware/acpi/platform_profile";

fn read_power_profile() -> profile::PowerProfile {
    let profile = fs::read_to_string(PLATFORM_PROFILE_PATH).unwrap();
    profile::PowerProfile::from(profile)
}

fn update_status(tray_handle: &ksni::Handle<tray::PerfProfileTray>) {
    let profile = read_power_profile();

    tray_handle.update(|tray| {
        tray.current_profile = profile.clone();
    });

    notify_rust::Notification::new()
        .summary("Platform profile changed")
        .body(format!("Current profile: {}", profile).as_str())
        .timeout(Timeout::Milliseconds(2_500))
        .show()
        .unwrap();
}

fn main() {
    let service = ksni::TrayService::new(tray::PerfProfileTray {
        current_profile: read_power_profile(),
    });
    let tray_handle = service.handle();
    service.spawn();

    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(_) => update_status(&tray_handle),
        Err(e) => println!("watch error: {:?}", e),
    })
    .unwrap();

    watcher
        .watch(
            Path::new(PLATFORM_PROFILE_PATH),
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    loop {
        thread::park()
    }
}
