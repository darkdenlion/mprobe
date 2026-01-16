#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::path::Path;

#[derive(Clone)]
pub struct BatteryInfo {
    pub percentage: f32,
    pub state: BatteryState,
    pub time_to_empty: Option<u64>,  // seconds
    pub time_to_full: Option<u64>,   // seconds
}

#[derive(Clone, Copy, PartialEq)]
pub enum BatteryState {
    Charging,
    Discharging,
    Full,
    NotCharging,
    Unknown,
}

impl std::fmt::Display for BatteryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BatteryState::Charging => write!(f, "Charging"),
            BatteryState::Discharging => write!(f, "Discharging"),
            BatteryState::Full => write!(f, "Full"),
            BatteryState::NotCharging => write!(f, "Not Charging"),
            BatteryState::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Default)]
pub struct BatteryData {
    pub batteries: Vec<BatteryInfo>,
    pub has_battery: bool,
}

impl BatteryData {
    pub fn update(&mut self) {
        self.batteries.clear();

        #[cfg(target_os = "macos")]
        self.update_macos();

        #[cfg(target_os = "linux")]
        self.update_linux();

        self.has_battery = !self.batteries.is_empty();
    }

    #[cfg(target_os = "macos")]
    fn update_macos(&mut self) {
        // Use ioreg to get battery info on macOS
        if let Ok(output) = std::process::Command::new("pmset")
            .args(["-g", "batt"])
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                self.parse_pmset_output(&stdout);
            }
        }
    }

    #[cfg(target_os = "macos")]
    fn parse_pmset_output(&mut self, output: &str) {
        // Example output:
        // Now drawing from 'Battery Power'
        //  -InternalBattery-0 (id=...)	95%; discharging; 5:30 remaining
        for line in output.lines() {
            if line.contains("InternalBattery") {
                let mut percentage = 0.0f32;
                let mut state = BatteryState::Unknown;
                let mut time_remaining: Option<u64> = None;

                // Extract percentage
                if let Some(pct_pos) = line.find('%') {
                    let start = line[..pct_pos].rfind(char::is_whitespace).map(|p| p + 1).unwrap_or(0);
                    if let Ok(pct) = line[start..pct_pos].trim().parse::<f32>() {
                        percentage = pct;
                    }
                }

                // Extract state
                if line.contains("charging;") && !line.contains("discharging") && !line.contains("not charging") {
                    state = BatteryState::Charging;
                } else if line.contains("discharging") {
                    state = BatteryState::Discharging;
                } else if line.contains("charged") || percentage >= 100.0 {
                    state = BatteryState::Full;
                } else if line.contains("not charging") {
                    state = BatteryState::NotCharging;
                }

                // Extract time remaining (format: "5:30 remaining" or "2:15 until charged")
                if let Some(remaining_pos) = line.find("remaining") {
                    let before = &line[..remaining_pos].trim();
                    if let Some(time_start) = before.rfind(';') {
                        let time_str = before[time_start + 1..].trim();
                        time_remaining = parse_time_str(time_str);
                    }
                } else if let Some(until_pos) = line.find("until charged") {
                    let before = &line[..until_pos].trim();
                    if let Some(time_start) = before.rfind(';') {
                        let time_str = before[time_start + 1..].trim();
                        time_remaining = parse_time_str(time_str);
                    }
                }

                let (time_to_empty, time_to_full) = match state {
                    BatteryState::Discharging => (time_remaining, None),
                    BatteryState::Charging => (None, time_remaining),
                    _ => (None, None),
                };

                self.batteries.push(BatteryInfo {
                    percentage,
                    state,
                    time_to_empty,
                    time_to_full,
                });
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn update_linux(&mut self) {
        let power_supply_path = Path::new("/sys/class/power_supply");

        if let Ok(entries) = fs::read_dir(power_supply_path) {
            for entry in entries.flatten() {
                let path = entry.path();

                // Check if this is a battery
                let type_path = path.join("type");
                if let Ok(type_content) = fs::read_to_string(&type_path) {
                    if type_content.trim() != "Battery" {
                        continue;
                    }
                }

                // Read capacity (percentage)
                let capacity_path = path.join("capacity");
                let percentage = fs::read_to_string(&capacity_path)
                    .ok()
                    .and_then(|s| s.trim().parse::<f32>().ok())
                    .unwrap_or(0.0);

                // Read status
                let status_path = path.join("status");
                let state = fs::read_to_string(&status_path)
                    .map(|s| match s.trim() {
                        "Charging" => BatteryState::Charging,
                        "Discharging" => BatteryState::Discharging,
                        "Full" => BatteryState::Full,
                        "Not charging" => BatteryState::NotCharging,
                        _ => BatteryState::Unknown,
                    })
                    .unwrap_or(BatteryState::Unknown);

                // Try to calculate time remaining
                let energy_now = read_sysfs_value(&path.join("energy_now"))
                    .or_else(|| read_sysfs_value(&path.join("charge_now")));
                let power_now = read_sysfs_value(&path.join("power_now"))
                    .or_else(|| read_sysfs_value(&path.join("current_now")));
                let energy_full = read_sysfs_value(&path.join("energy_full"))
                    .or_else(|| read_sysfs_value(&path.join("charge_full")));

                let (time_to_empty, time_to_full) = match (state, energy_now, power_now, energy_full) {
                    (BatteryState::Discharging, Some(energy), Some(power), _) if power > 0 => {
                        (Some((energy as f64 / power as f64 * 3600.0) as u64), None)
                    }
                    (BatteryState::Charging, Some(energy), Some(power), Some(full)) if power > 0 => {
                        let remaining = full.saturating_sub(energy);
                        (None, Some((remaining as f64 / power as f64 * 3600.0) as u64))
                    }
                    _ => (None, None),
                };

                self.batteries.push(BatteryInfo {
                    percentage,
                    state,
                    time_to_empty,
                    time_to_full,
                });
            }
        }
    }

    pub fn format_time(seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;

        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }
}

fn parse_time_str(time_str: &str) -> Option<u64> {
    // Parse "5:30" format to seconds
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() == 2 {
        let hours = parts[0].trim().parse::<u64>().ok()?;
        let minutes = parts[1].trim().parse::<u64>().ok()?;
        Some(hours * 3600 + minutes * 60)
    } else {
        None
    }
}

#[cfg(target_os = "linux")]
fn read_sysfs_value(path: &Path) -> Option<u64> {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| s.trim().parse().ok())
}
