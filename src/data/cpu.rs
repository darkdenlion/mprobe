use sysinfo::System;

#[derive(Default)]
pub struct CpuData {
    pub total_usage: f64,
    pub per_core_usage: Vec<f64>,
    pub core_count: usize,
    pub cpu_name: String,
    pub frequency: u64,
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

        // Per-core usage
        self.per_core_usage = cpus.iter().map(|cpu| cpu.cpu_usage() as f64).collect();
    }

    pub fn get_usage_color_index(&self) -> usize {
        match self.total_usage {
            x if x < 30.0 => 0, // Green
            x if x < 60.0 => 1, // Yellow
            x if x < 85.0 => 2, // Orange
            _ => 3,             // Red
        }
    }
}
