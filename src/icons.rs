use std::{fs, path::Path};

use crate::profile::PowerProfile;

const ICON_PATH: &str = ".local/share/icons/perfprofile/scalable/status";

pub trait ToIcon {
    fn to_icon_data(&self) -> &'static [u8];
    fn to_icon_name(&self) -> String;
}

impl ToIcon for PowerProfile {
    // Low -> Leaf (Power saving)
    // Med -> Tilde (Balanced)
    // High -> Bolt (Energy, fast, powerful)

    fn to_icon_data(&self) -> &'static [u8] {
        match self {
            PowerProfile::Performance => include_bytes!("../resources/high_voltage.svg"),
            PowerProfile::Balanced => include_bytes!("../resources/light_bulb.svg"),
            PowerProfile::LowPower => include_bytes!("../resources/leaf.svg"),
        }
    }

    fn to_icon_name(&self) -> String {
        match self {
            PowerProfile::Performance => "high_voltage".to_owned(),
            PowerProfile::Balanced => "light_bulb".to_owned(),
            PowerProfile::LowPower => "leaf".to_owned(),
        }
    }
}

pub fn install() {
    let userdirs = directories::UserDirs::new().unwrap();

    let icons_dir_path = Path::new(userdirs.home_dir()).join(ICON_PATH);
    if !icons_dir_path.exists() {
        fs::create_dir_all(icons_dir_path).unwrap();
    }

    let profiles = vec![
        PowerProfile::Performance,
        PowerProfile::Balanced,
        PowerProfile::LowPower,
    ];

    for profile in profiles {
        let icon_path = Path::new(userdirs.home_dir())
            .join(ICON_PATH)
            .join(profile.to_icon_name())
            .with_extension(".svg");

        if !icon_path.exists() {
            fs::write(icon_path, profile.to_icon_data())
                .unwrap_or_else(|_| panic!("> Unable to install custom app icons\nRun me as root first or copy /resources to {ICON_PATH}"))`;
        }
    }
}
