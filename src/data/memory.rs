use sysinfo::System;

#[derive(Default)]
pub struct MemoryData {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub free: u64,
    pub used_percent: f64,
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_free: u64,
    pub swap_percent: f64,
    // Additional breakdown (where available)
    pub cached: u64,
    pub buffers: u64,
}

impl MemoryData {
    pub fn update(&mut self, system: &System) {
        self.total = system.total_memory();
        self.used = system.used_memory();
        self.available = system.available_memory();
        self.free = system.free_memory();

        if self.total > 0 {
            self.used_percent = (self.used as f64 / self.total as f64) * 100.0;
        }

        self.swap_total = system.total_swap();
        self.swap_used = system.used_swap();
        self.swap_free = system.free_swap();

        if self.swap_total > 0 {
            self.swap_percent = (self.swap_used as f64 / self.swap_total as f64) * 100.0;
        }

        // On Linux, try to get cached/buffers from /proc/meminfo
        #[cfg(target_os = "linux")]
        self.update_linux_details();

        // On macOS, calculate estimated cached from difference
        #[cfg(target_os = "macos")]
        {
            // available = free + cached (approximately)
            // So cached ≈ available - free
            if self.available > self.free {
                self.cached = self.available - self.free;
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn update_linux_details(&mut self) {
        if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
            for line in content.lines() {
                if line.starts_with("Cached:") {
                    self.cached = parse_meminfo_value(line);
                } else if line.starts_with("Buffers:") {
                    self.buffers = parse_meminfo_value(line);
                }
            }
        }
    }

    pub fn format_bytes(bytes: u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        let bytes = bytes as f64;
        if bytes >= GB {
            format!("{:.1} GB", bytes / GB)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes / MB)
        } else if bytes >= KB {
            format!("{:.1} KB", bytes / KB)
        } else {
            format!("{:.0} B", bytes)
        }
    }
}

#[cfg(target_os = "linux")]
fn parse_meminfo_value(line: &str) -> u64 {
    // Format: "Cached:          1234567 kB"
    line.split_whitespace()
        .nth(1)
        .and_then(|s| s.parse::<u64>().ok())
        .map(|kb| kb * 1024) // Convert from kB to bytes
        .unwrap_or(0)
}
