use sysinfo::System;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Pid,
    Name,
    Cpu,
    Memory,
}

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub memory_percent: f64,
    pub status: String,
    pub user: String,
    pub cmd: String,
}

#[derive(Default)]
pub struct ProcessData {
    pub processes: Vec<ProcessInfo>,
    pub total_processes: usize,
    pub running_processes: usize,
}

impl ProcessData {
    pub fn update(
        &mut self,
        system: &System,
        filter: &str,
        sort_column: SortColumn,
        sort_ascending: bool,
    ) {
        let total_memory = system.total_memory();

        self.processes.clear();
        self.total_processes = 0;
        self.running_processes = 0;

        for (pid, process) in system.processes() {
            self.total_processes += 1;

            let status = format!("{:?}", process.status());
            if status.contains("Run") {
                self.running_processes += 1;
            }

            let name = process.name().to_string_lossy().to_string();
            let cmd = process
                .cmd()
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join(" ");

            // Apply filter
            if !filter.is_empty() {
                let filter_lower = filter.to_lowercase();
                let name_lower = name.to_lowercase();
                let cmd_lower = cmd.to_lowercase();

                if !name_lower.contains(&filter_lower) && !cmd_lower.contains(&filter_lower) {
                    continue;
                }
            }

            let memory = process.memory();
            let memory_percent = if total_memory > 0 {
                (memory as f64 / total_memory as f64) * 100.0
            } else {
                0.0
            };

            self.processes.push(ProcessInfo {
                pid: pid.as_u32(),
                name,
                cpu_usage: process.cpu_usage(),
                memory,
                memory_percent,
                status,
                user: String::new(), // User info requires additional setup on some platforms
                cmd,
            });
        }

        // Sort processes
        self.processes.sort_by(|a, b| {
            let cmp = match sort_column {
                SortColumn::Pid => a.pid.cmp(&b.pid),
                SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                SortColumn::Cpu => a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
                SortColumn::Memory => a.memory.cmp(&b.memory),
            };

            if sort_ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
    }

    pub fn format_memory(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;

        if bytes >= GB {
            format!("{:.1}G", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1}M", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.0}K", bytes as f64 / KB as f64)
        } else {
            format!("{}B", bytes)
        }
    }
}
