use crate::profile::PowerProfile;

fn profile_icon(profile: &PowerProfile) -> String {
    match profile {
        PowerProfile::Performance => "🚀".to_string(),
        PowerProfile::Balanced => "🔋".to_string(),
        PowerProfile::LowPower => "🌙".to_string(),
    }
}

pub struct PerfProfileTray {
    pub current_profile: PowerProfile,
}

impl ksni::Tray for PerfProfileTray {
    fn title(&self) -> String {
        format!("Current Profile: {}", self.current_profile).to_string()
    }

    fn icon_name(&self) -> String {
        // Low -> Leaf (Power saving)
        // Med -> Tilde (Balanced)
        // High -> Bolt (Energy, fast, powerful)

        profile_icon(&self.current_profile).into()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;

        vec![StandardItem {
            label: "Exit".into(),
            activate: Box::new(|_| std::process::exit(0)),
            ..Default::default()
        }
        .into()]
    }
}
