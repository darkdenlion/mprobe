use sysinfo::System;
use std::collections::HashMap;

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
    pub parent_pid: Option<u32>,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub memory_percent: f64,
    pub status: String,
    pub depth: usize,  // For tree view indentation
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
        tree_view: bool,
    ) {
        let total_memory = system.total_memory();

        self.processes.clear();
        self.total_processes = 0;
        self.running_processes = 0;

        let mut all_processes: Vec<ProcessInfo> = Vec::new();

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

            let parent_pid = process.parent().map(|p| p.as_u32());

            all_processes.push(ProcessInfo {
                pid: pid.as_u32(),
                parent_pid,
                name,
                cpu_usage: process.cpu_usage(),
                memory,
                memory_percent,
                status,
                depth: 0,
            });
        }

        if tree_view && filter.is_empty() {
            // Build tree structure
            self.processes = build_process_tree(all_processes);
        } else {
            // Flat list with sorting
            self.processes = all_processes;

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

fn build_process_tree(mut processes: Vec<ProcessInfo>) -> Vec<ProcessInfo> {
    // Build a map of pid -> children
    let mut children_map: HashMap<u32, Vec<usize>> = HashMap::new();
    let mut root_indices: Vec<usize> = Vec::new();

    // Create a set of all PIDs for quick lookup
    let all_pids: std::collections::HashSet<u32> = processes.iter().map(|p| p.pid).collect();

    for (i, proc) in processes.iter().enumerate() {
        if let Some(ppid) = proc.parent_pid {
            if all_pids.contains(&ppid) {
                children_map.entry(ppid).or_default().push(i);
            } else {
                // Parent not in list, treat as root
                root_indices.push(i);
            }
        } else {
            root_indices.push(i);
        }
    }

    // Sort roots by CPU usage (descending)
    root_indices.sort_by(|&a, &b| {
        processes[b].cpu_usage.partial_cmp(&processes[a].cpu_usage)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // Build flat tree list with depth
    let mut result: Vec<ProcessInfo> = Vec::new();

    fn add_with_children(
        idx: usize,
        depth: usize,
        processes: &mut Vec<ProcessInfo>,
        children_map: &HashMap<u32, Vec<usize>>,
        result: &mut Vec<ProcessInfo>,
        visited: &mut std::collections::HashSet<usize>,
    ) {
        if visited.contains(&idx) {
            return;
        }
        visited.insert(idx);

        let mut proc = processes[idx].clone();
        proc.depth = depth;
        let pid = proc.pid;
        result.push(proc);

        if let Some(children) = children_map.get(&pid) {
            let mut sorted_children = children.clone();
            // Sort children by CPU usage
            sorted_children.sort_by(|&a, &b| {
                processes[b].cpu_usage.partial_cmp(&processes[a].cpu_usage)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            for &child_idx in &sorted_children {
                add_with_children(child_idx, depth + 1, processes, children_map, result, visited);
            }
        }
    }

    let mut visited = std::collections::HashSet::new();
    for &root_idx in &root_indices {
        add_with_children(root_idx, 0, &mut processes, &children_map, &mut result, &mut visited);
    }

    result
}
