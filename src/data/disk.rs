use sysinfo::Disks;

pub struct DiskData {
    disks: Disks,
    pub disk_info: Vec<DiskInfo>,
}

pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub used_percent: f64,
    pub fs_type: String,
}

impl Default for DiskData {
    fn default() -> Self {
        Self {
            disks: Disks::new_with_refreshed_list(),
            disk_info: Vec::new(),
        }
    }
}

impl DiskData {
    pub fn update(&mut self) {
        self.disks.refresh();
        self.disk_info.clear();

        for disk in self.disks.iter() {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total.saturating_sub(available);
            let used_percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            let name = disk.name().to_string_lossy().to_string();
            let display_name = if name.is_empty() {
                disk.mount_point().to_string_lossy().to_string()
            } else {
                name
            };

            self.disk_info.push(DiskInfo {
                name: display_name,
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total,
                used,
                available,
                used_percent,
                fs_type: disk.file_system().to_string_lossy().to_string(),
            });
        }

        // Sort by mount point for consistent ordering
        self.disk_info.sort_by(|a, b| a.mount_point.cmp(&b.mount_point));
    }

    pub fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if bytes >= TB {
            format!("{:.1}T", bytes as f64 / TB as f64)
        } else if bytes >= GB {
            format!("{:.1}G", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1}M", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.1}K", bytes as f64 / KB as f64)
        } else {
            format!("{}B", bytes)
        }
    }
}
