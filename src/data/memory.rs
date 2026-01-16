use sysinfo::System;

#[derive(Default)]
pub struct MemoryData {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub used_percent: f64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_percent: f64,
}

impl MemoryData {
    pub fn update(&mut self, system: &System) {
        self.total = system.total_memory();
        self.used = system.used_memory();
        self.available = system.available_memory();

        if self.total > 0 {
            self.used_percent = (self.used as f64 / self.total as f64) * 100.0;
        }

        self.swap_total = system.total_swap();
        self.swap_used = system.used_swap();

        if self.swap_total > 0 {
            self.swap_percent = (self.swap_used as f64 / self.swap_total as f64) * 100.0;
        }
    }

    pub fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if bytes >= TB {
            format!("{:.1} TB", bytes as f64 / TB as f64)
        } else if bytes >= GB {
            format!("{:.1} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.1} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }

    pub fn get_usage_color_index(&self) -> usize {
        match self.used_percent {
            x if x < 50.0 => 0, // Green
            x if x < 70.0 => 1, // Yellow
            x if x < 85.0 => 2, // Orange
            _ => 3,             // Red
        }
    }
}
