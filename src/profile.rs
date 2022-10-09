use std::fmt::Display;

pub enum PowerProfile {
    Performance,
    Balanced,
    LowPower,
}

impl From<String> for PowerProfile {
    fn from(profile: String) -> Self {
        match profile.trim() {
            "performance" => PowerProfile::Performance,
            "balanced" => PowerProfile::Balanced,
            "low-power" => PowerProfile::LowPower,
            _ => panic!("Unknown power profile"),
        }
    }
}

impl Display for PowerProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PowerProfile::Performance => write!(f, "Performance"),
            PowerProfile::Balanced => write!(f, "Balanced"),
            PowerProfile::LowPower => write!(f, "Low Power"),
        }
    }
}

impl Clone for PowerProfile {
    fn clone(&self) -> Self {
        match self {
            PowerProfile::Performance => PowerProfile::Performance,
            PowerProfile::Balanced => PowerProfile::Balanced,
            PowerProfile::LowPower => PowerProfile::LowPower,
        }
    }
}
