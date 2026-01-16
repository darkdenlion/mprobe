use sysinfo::System;

#[derive(Default)]
pub struct CpuData {
    pub total_usage: f64,
    pub per_core_usage: Vec<f64>,
    pub per_core_freq: Vec<u64>,
    pub core_count: usize,
    pub cpu_name: String,
    pub frequency: u64,
    pub frequency_min: u64,
    pub frequency_max: u64,
}

impl CpuData {
    pub fn update(&mut self, system: &System) {
        let cpus = system.cpus();

        if !cpus.is_empty() {
            self.cpu_name = cpus[0].brand().to_string();
            self.frequency = cpus[0].frequency();
        }

        self.core_count = cpus.len();

        // Calculate total CPU usage (average of all cores)
        let total: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        self.total_usage = (total / cpus.len() as f32) as f64;

        // Per-core usage and frequency
        self.per_core_usage = cpus.iter().map(|cpu| cpu.cpu_usage() as f64).collect();
        self.per_core_freq = cpus.iter().map(|cpu| cpu.frequency()).collect();

        // Calculate min/max frequency
        if !self.per_core_freq.is_empty() {
            self.frequency_min = *self.per_core_freq.iter().min().unwrap_or(&0);
            self.frequency_max = *self.per_core_freq.iter().max().unwrap_or(&0);
        }
    }

    pub fn format_frequency(freq_mhz: u64) -> String {
        if freq_mhz >= 1000 {
            format!("{:.2} GHz", freq_mhz as f64 / 1000.0)
        } else {
            format!("{} MHz", freq_mhz)
        }
    }
}
