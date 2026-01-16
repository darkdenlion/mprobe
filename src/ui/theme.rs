use ratatui::style::Color;

pub struct Theme {
    // Base colors
    pub bg: Color,
    pub bg_secondary: Color,
    pub fg: Color,
    pub fg_dim: Color,
    pub fg_muted: Color,

    // Accent colors
    pub accent: Color,

    // Border colors
    pub border: Color,

    // Semantic colors
    pub success: Color,
    pub warning: Color,

    // Component specific
    pub cpu_color: Color,
    pub mem_color: Color,
    pub swap_color: Color,
    pub net_up: Color,
    pub net_down: Color,
    pub disk_color: Color,

    // Graph colors (gradient from low to high usage)
    pub usage_low: Color,
    pub usage_medium: Color,
    pub usage_high: Color,
    pub usage_critical: Color,

    // Table colors
    pub table_header: Color,
    pub table_row_alt: Color,
    pub table_selected: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // Adwaita Dark theme
            bg: Color::Rgb(30, 30, 30),             // #1e1e1e
            bg_secondary: Color::Rgb(45, 45, 45),   // #2d2d2d
            fg: Color::Rgb(255, 255, 255),          // #ffffff
            fg_dim: Color::Rgb(154, 153, 150),      // #9a9996
            fg_muted: Color::Rgb(119, 118, 123),    // #77767b

            // Adwaita blue accent
            accent: Color::Rgb(53, 132, 228),       // #3584e4

            // Borders
            border: Color::Rgb(69, 69, 69),         // #454545

            // Semantic colors
            success: Color::Rgb(51, 209, 122),      // #33d17a
            warning: Color::Rgb(229, 165, 10),      // #e5a50a

            // Component colors (Adwaita palette)
            cpu_color: Color::Rgb(53, 132, 228),    // #3584e4 Blue
            mem_color: Color::Rgb(145, 65, 172),    // #9141ac Purple
            swap_color: Color::Rgb(192, 97, 203),   // #c061cb Pink
            net_up: Color::Rgb(51, 209, 122),       // #33d17a Green
            net_down: Color::Rgb(53, 132, 228),     // #3584e4 Blue
            disk_color: Color::Rgb(255, 120, 0),    // #ff7800 Orange

            // Usage gradient (Adwaita colors)
            usage_low: Color::Rgb(51, 209, 122),    // #33d17a Green
            usage_medium: Color::Rgb(246, 211, 45), // #f6d32d Yellow
            usage_high: Color::Rgb(255, 120, 0),    // #ff7800 Orange
            usage_critical: Color::Rgb(224, 27, 36),// #e01b24 Red

            // Table
            table_header: Color::Rgb(154, 153, 150),// #9a9996
            table_row_alt: Color::Rgb(38, 38, 38),  // #262626
            table_selected: Color::Rgb(53, 53, 53), // #353535
        }
    }
}

impl Theme {
    pub fn get_usage_color(&self, percent: f64) -> Color {
        match percent {
            x if x < 50.0 => self.usage_low,
            x if x < 75.0 => self.usage_medium,
            x if x < 90.0 => self.usage_high,
            _ => self.usage_critical,
        }
    }

    pub fn format_bytes(&self, bytes: u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;
        const TB: f64 = GB * 1024.0;

        let bytes = bytes as f64;
        if bytes >= TB {
            format!("{:.1} TB", bytes / TB)
        } else if bytes >= GB {
            format!("{:.1} GB", bytes / GB)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes / MB)
        } else if bytes >= KB {
            format!("{:.1} KB", bytes / KB)
        } else {
            format!("{:.0} B", bytes)
        }
    }

    pub fn format_speed(&self, bytes_per_sec: u64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        let b = bytes_per_sec as f64;
        if b >= GB {
            format!("{:.1} GB/s", b / GB)
        } else if b >= MB {
            format!("{:.1} MB/s", b / MB)
        } else if b >= KB {
            format!("{:.1} KB/s", b / KB)
        } else {
            format!("{:.0} B/s", b)
        }
    }
}
