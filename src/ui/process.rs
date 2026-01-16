use crate::app::App;
use crate::data::{ProcessData, SortColumn};
use crate::ui::Theme;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let filter_indicator = if app.filter_mode {
        format!(" Filter: {}█", app.filter_text)
    } else if !app.filter_text.is_empty() {
        format!(" Filter: {}", app.filter_text)
    } else {
        String::new()
    };

    let sort_indicator = match app.sort_column {
        SortColumn::Pid => "PID",
        SortColumn::Name => "NAME",
        SortColumn::Cpu => "CPU%",
        SortColumn::Memory => "MEM",
    };

    let sort_arrow = if app.sort_ascending { "↑" } else { "↓" };

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(
                "PROCESSES",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" ({}) ", app.process_data.processes.len()),
                Style::default().fg(theme.fg_muted),
            ),
            Span::styled(
                format!("[{} {}]", sort_indicator, sort_arrow),
                Style::default().fg(theme.fg_dim),
            ),
            Span::styled(
                filter_indicator,
                Style::default()
                    .fg(theme.warning)
                    .add_modifier(Modifier::BOLD),
            ),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg_secondary));

    // Table header
    let header_cells = ["  PID", "NAME", "CPU%", "MEMORY", "STATUS"]
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let is_sorted = (i == 0 && app.sort_column == SortColumn::Pid)
                || (i == 1 && app.sort_column == SortColumn::Name)
                || (i == 2 && app.sort_column == SortColumn::Cpu)
                || (i == 3 && app.sort_column == SortColumn::Memory);

            let style = if is_sorted {
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.table_header)
            };
            Cell::from(*h).style(style)
        });

    let header = Row::new(header_cells)
        .style(Style::default().bg(theme.bg_secondary))
        .height(1)
        .bottom_margin(0);

    // Table rows
    let processes = app.get_filtered_processes();
    let rows = processes.iter().enumerate().map(|(i, proc)| {
        let cpu_color = theme.get_usage_color(proc.cpu_usage as f64);
        let mem_color = theme.get_usage_color(proc.memory_percent);

        let selected = i == app.process_scroll;
        let row_bg = if selected {
            theme.table_selected
        } else if i % 2 == 1 {
            theme.table_row_alt
        } else {
            theme.bg_secondary
        };

        let status_color = if proc.status.contains("Run") {
            theme.success
        } else {
            theme.fg_muted
        };

        // Create name cell with highlighted filter matches
        let name_cell = if !app.filter_text.is_empty() {
            let name = truncate_string(&proc.name, 25);
            Cell::from(highlight_matches(&name, &app.filter_text, theme.fg, theme.warning))
        } else {
            Cell::from(truncate_string(&proc.name, 25)).style(Style::default().fg(theme.fg))
        };

        let cells = vec![
            Cell::from(format!("{:>6}", proc.pid)).style(Style::default().fg(theme.fg_dim)),
            name_cell,
            Cell::from(format!("{:>6.1}", proc.cpu_usage))
                .style(Style::default().fg(cpu_color)),
            Cell::from(format!("{:>8}", ProcessData::format_memory(proc.memory)))
                .style(Style::default().fg(mem_color)),
            Cell::from(format_status(&proc.status)).style(Style::default().fg(status_color)),
        ];

        Row::new(cells).style(Style::default().bg(row_bg))
    });

    let widths = [
        Constraint::Length(8),   // PID
        Constraint::Min(20),     // Name
        Constraint::Length(8),   // CPU
        Constraint::Length(10),  // Memory
        Constraint::Length(10),  // Status
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .row_highlight_style(
            Style::default()
                .bg(theme.table_selected)
                .add_modifier(Modifier::BOLD),
        )
        .style(Style::default().bg(theme.bg_secondary));

    let mut state = TableState::default();
    state.select(Some(app.process_scroll));

    frame.render_stateful_widget(table, area, &mut state);
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}…", &s[..max_len - 1])
    } else {
        s.to_string()
    }
}

fn format_status(status: &str) -> String {
    if status.contains("Run") {
        "Running".to_string()
    } else if status.contains("Sleep") {
        "Sleep".to_string()
    } else if status.contains("Idle") {
        "Idle".to_string()
    } else if status.contains("Zombie") {
        "Zombie".to_string()
    } else {
        status.chars().take(7).collect()
    }
}

/// Highlight matching text in a string
fn highlight_matches(text: &str, query: &str, normal_color: Color, highlight_color: Color) -> Line<'static> {
    if query.is_empty() {
        return Line::from(Span::styled(text.to_string(), Style::default().fg(normal_color)));
    }

    let query_lower = query.to_lowercase();
    let text_lower = text.to_lowercase();

    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut last_end = 0;

    // Find all occurrences of the query in the text (case-insensitive)
    for (start, _) in text_lower.match_indices(&query_lower) {
        // Add non-matching part before this match
        if start > last_end {
            spans.push(Span::styled(
                text[last_end..start].to_string(),
                Style::default().fg(normal_color),
            ));
        }

        // Add the matching part with highlight
        let end = start + query.len();
        spans.push(Span::styled(
            text[start..end].to_string(),
            Style::default()
                .fg(highlight_color)
                .add_modifier(Modifier::BOLD),
        ));

        last_end = end;
    }

    // Add remaining non-matching part
    if last_end < text.len() {
        spans.push(Span::styled(
            text[last_end..].to_string(),
            Style::default().fg(normal_color),
        ));
    }

    // If no matches found, return the whole text in normal color
    if spans.is_empty() {
        spans.push(Span::styled(text.to_string(), Style::default().fg(normal_color)));
    }

    Line::from(spans)
}
