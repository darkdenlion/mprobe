use ratatui::style::Color;

pub struct Theme {
    // Base colors
    pub bg: Color,
    pub bg_secondary: Color,
    pub bg_highlight: Color,
    pub fg: Color,
    pub fg_dim: Color,
    pub fg_muted: Color,

    // Accent colors
    pub accent: Color,
    pub accent_secondary: Color,

    // Border colors
    pub border: Color,
    pub border_focused: Color,

    // Semantic colors
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,

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
            // Modern dark theme with slight blue tint
            bg: Color::Rgb(13, 17, 23),
            bg_secondary: Color::Rgb(22, 27, 34),
            bg_highlight: Color::Rgb(33, 38, 45),
            fg: Color::Rgb(230, 237, 243),
            fg_dim: Color::Rgb(139, 148, 158),
            fg_muted: Color::Rgb(110, 118, 129),

            // Cyan/Teal accent (modern and professional)
            accent: Color::Rgb(88, 166, 255),
            accent_secondary: Color::Rgb(56, 139, 253),

            // Subtle borders
            border: Color::Rgb(48, 54, 61),
            border_focused: Color::Rgb(88, 166, 255),

            // Semantic colors
            success: Color::Rgb(63, 185, 80),
            warning: Color::Rgb(210, 153, 34),
            error: Color::Rgb(248, 81, 73),
            info: Color::Rgb(88, 166, 255),

            // Component colors
            cpu_color: Color::Rgb(88, 166, 255),    // Blue
            mem_color: Color::Rgb(163, 113, 247),   // Purple
            swap_color: Color::Rgb(219, 97, 162),   // Pink
            net_up: Color::Rgb(63, 185, 80),        // Green
            net_down: Color::Rgb(88, 166, 255),     // Blue
            disk_color: Color::Rgb(210, 153, 34),   // Yellow/Orange

            // Usage gradient
            usage_low: Color::Rgb(63, 185, 80),     // Green
            usage_medium: Color::Rgb(210, 153, 34), // Yellow
            usage_high: Color::Rgb(219, 97, 162),   // Pink
            usage_critical: Color::Rgb(248, 81, 73),// Red

            // Table
            table_header: Color::Rgb(139, 148, 158),
            table_row_alt: Color::Rgb(22, 27, 34),
            table_selected: Color::Rgb(33, 38, 45),
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

    pub fn sparkline_from_data(&self, data: &[f64], width: usize) -> String {
        let blocks = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        if data.is_empty() {
            return " ".repeat(width);
        }

        let max_val = data.iter().cloned().fold(f64::MIN, f64::max).max(1.0);
        let min_val = data.iter().cloned().fold(f64::MAX, f64::min).min(0.0);
        let range = (max_val - min_val).max(1.0);

        let start = data.len().saturating_sub(width);
        data[start..]
            .iter()
            .map(|&v| {
                let normalized = ((v - min_val) / range).clamp(0.0, 1.0);
                let idx = (normalized * 7.0) as usize;
                blocks[idx.min(7)]
            })
            .collect()
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
