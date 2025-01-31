use crate::{icons::ToIcon, profile::PowerProfile};
use ksni::menu::StandardItem;

pub struct PerfProfileTray {
    pub current_profile: PowerProfile,
}

impl ksni::Tray for PerfProfileTray {
    fn title(&self) -> String {
        format!("Current Profile: {}", self.current_profile)
    }

    fn icon_name(&self) -> String {
        self.current_profile.to_icon_name()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        vec![StandardItem {
            label: "Exit".into(),
            activate: Box::new(|_| std::process::exit(0)),
            ..Default::default()
        }
        .into()]
    }
}
