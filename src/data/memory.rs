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
}
