use crate::data::{CpuData, MemoryData, NetworkData, ProcessData, ProcessInfo, SortColumn};
use sysinfo::System;
use std::collections::VecDeque;

const GRAPH_HISTORY_SIZE: usize = 120;

pub struct App {
    pub system: System,
    pub cpu_data: CpuData,
    pub memory_data: MemoryData,
    pub network_data: NetworkData,
    pub process_data: ProcessData,
    pub cpu_history: VecDeque<f64>,
    pub mem_history: VecDeque<f64>,
    pub net_up_history: VecDeque<u64>,
    pub net_down_history: VecDeque<u64>,
    pub selected_tab: usize,
    pub process_scroll: usize,
    pub filter_mode: bool,
    pub filter_text: String,
    pub tree_view: bool,
    pub sort_column: SortColumn,
    pub sort_ascending: bool,
    pub hostname: String,
    pub os_name: String,
    pub kernel_version: String,
    pub uptime: u64,
}

impl App {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());
        let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());

        let mut app = Self {
            system,
            cpu_data: CpuData::default(),
            memory_data: MemoryData::default(),
            network_data: NetworkData::default(),
            process_data: ProcessData::default(),
            cpu_history: VecDeque::with_capacity(GRAPH_HISTORY_SIZE),
            mem_history: VecDeque::with_capacity(GRAPH_HISTORY_SIZE),
            net_up_history: VecDeque::with_capacity(GRAPH_HISTORY_SIZE),
            net_down_history: VecDeque::with_capacity(GRAPH_HISTORY_SIZE),
            selected_tab: 0,
            process_scroll: 0,
            filter_mode: false,
            filter_text: String::new(),
            tree_view: false,
            sort_column: SortColumn::Cpu,
            sort_ascending: false,
            hostname,
            os_name,
            kernel_version,
            uptime: 0,
        };

        // Initialize with zeros
        for _ in 0..GRAPH_HISTORY_SIZE {
            app.cpu_history.push_back(0.0);
            app.mem_history.push_back(0.0);
            app.net_up_history.push_back(0);
            app.net_down_history.push_back(0);
        }

        app.update();
        app
    }

    pub fn update(&mut self) {
        self.system.refresh_all();
        self.uptime = System::uptime();

        // Update CPU data
        self.cpu_data.update(&self.system);
        if self.cpu_history.len() >= GRAPH_HISTORY_SIZE {
            self.cpu_history.pop_front();
        }
        self.cpu_history.push_back(self.cpu_data.total_usage);

        // Update Memory data
        self.memory_data.update(&self.system);
        if self.mem_history.len() >= GRAPH_HISTORY_SIZE {
            self.mem_history.pop_front();
        }
        self.mem_history.push_back(self.memory_data.used_percent);

        // Update Network data
        let (up, down) = self.network_data.update(&self.system);
        if self.net_up_history.len() >= GRAPH_HISTORY_SIZE {
            self.net_up_history.pop_front();
        }
        if self.net_down_history.len() >= GRAPH_HISTORY_SIZE {
            self.net_down_history.pop_front();
        }
        self.net_up_history.push_back(up);
        self.net_down_history.push_back(down);

        // Update Process data
        self.process_data.update(&self.system, &self.filter_text, self.sort_column, self.sort_ascending);
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 4;
    }

    pub fn prev_tab(&mut self) {
        self.selected_tab = if self.selected_tab == 0 { 3 } else { self.selected_tab - 1 };
    }

    pub fn scroll_up(&mut self) {
        if self.process_scroll > 0 {
            self.process_scroll -= 1;
        }
    }

    pub fn scroll_down(&mut self) {
        let max_scroll = self.process_data.processes.len().saturating_sub(1);
        if self.process_scroll < max_scroll {
            self.process_scroll += 1;
        }
    }

    pub fn scroll_to_top(&mut self) {
        self.process_scroll = 0;
    }

    pub fn scroll_to_bottom(&mut self) {
        self.process_scroll = self.process_data.processes.len().saturating_sub(1);
    }

    pub fn toggle_filter_mode(&mut self) {
        self.filter_mode = !self.filter_mode;
    }

    pub fn add_filter_char(&mut self, c: char) {
        self.filter_text.push(c);
        self.process_scroll = 0;
    }

    pub fn remove_filter_char(&mut self) {
        self.filter_text.pop();
        self.process_scroll = 0;
    }

    pub fn clear_filter(&mut self) {
        self.filter_text.clear();
        self.filter_mode = false;
        self.process_scroll = 0;
    }

    pub fn toggle_tree_view(&mut self) {
        self.tree_view = !self.tree_view;
    }

    pub fn cycle_sort(&mut self) {
        self.sort_column = match self.sort_column {
            SortColumn::Pid => SortColumn::Name,
            SortColumn::Name => SortColumn::Cpu,
            SortColumn::Cpu => SortColumn::Memory,
            SortColumn::Memory => SortColumn::Pid,
        };
        self.process_scroll = 0;
    }

    pub fn toggle_sort_order(&mut self) {
        self.sort_ascending = !self.sort_ascending;
        self.process_scroll = 0;
    }

    pub fn get_filtered_processes(&self) -> &[ProcessInfo] {
        &self.process_data.processes
    }

    pub fn format_uptime(&self) -> String {
        let days = self.uptime / 86400;
        let hours = (self.uptime % 86400) / 3600;
        let minutes = (self.uptime % 3600) / 60;

        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }
}
