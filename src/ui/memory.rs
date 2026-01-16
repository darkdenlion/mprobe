use crate::app::App;
use crate::ui::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(
                "MEMORY",
                Style::default()
                    .fg(theme.mem_color)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" ", Style::default()),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg_secondary));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1), // RAM stats
            Constraint::Length(2), // RAM bar
            Constraint::Length(1), // Swap stats
            Constraint::Length(2), // Swap bar
            Constraint::Min(1),    // Graph
        ])
        .split(inner);

    let ram_color = theme.get_usage_color(app.memory_data.used_percent);
    let swap_color = theme.get_usage_color(app.memory_data.swap_percent);

    // RAM stats
    let ram_stats = Line::from(vec![
        Span::styled("RAM ", Style::default().fg(theme.fg_muted)),
        Span::styled(
            format!("{:.1}%", app.memory_data.used_percent),
            Style::default().fg(ram_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!(
                "{} / {}",
                theme.format_bytes(app.memory_data.used),
                theme.format_bytes(app.memory_data.total)
            ),
            Style::default().fg(theme.fg_dim),
        ),
    ]);
    frame.render_widget(Paragraph::new(ram_stats), chunks[0]);

    // RAM bar
    let bar_width = chunks[1].width.saturating_sub(2) as usize;
    let ram_filled = ((app.memory_data.used_percent / 100.0) * bar_width as f64) as usize;
    let ram_empty = bar_width.saturating_sub(ram_filled);

    let ram_bar = Line::from(vec![
        Span::styled("▓".repeat(ram_filled), Style::default().fg(ram_color)),
        Span::styled("░".repeat(ram_empty), Style::default().fg(theme.border)),
    ]);
    frame.render_widget(Paragraph::new(ram_bar), chunks[1]);

    // Swap stats
    let swap_stats = Line::from(vec![
        Span::styled("SWP ", Style::default().fg(theme.fg_muted)),
        Span::styled(
            format!("{:.1}%", app.memory_data.swap_percent),
            Style::default().fg(swap_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!(
                "{} / {}",
                theme.format_bytes(app.memory_data.swap_used),
                theme.format_bytes(app.memory_data.swap_total)
            ),
            Style::default().fg(theme.fg_dim),
        ),
    ]);
    frame.render_widget(Paragraph::new(swap_stats), chunks[2]);

    // Swap bar
    let swap_filled = ((app.memory_data.swap_percent / 100.0) * bar_width as f64) as usize;
    let swap_empty = bar_width.saturating_sub(swap_filled);

    let swap_bar = Line::from(vec![
        Span::styled("▓".repeat(swap_filled), Style::default().fg(theme.swap_color)),
        Span::styled("░".repeat(swap_empty), Style::default().fg(theme.border)),
    ]);
    frame.render_widget(Paragraph::new(swap_bar), chunks[3]);

    // Mini graph
    let data: Vec<(f64, f64)> = app
        .mem_history
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v))
        .collect();

    if chunks[4].height >= 2 {
        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line)
            .style(Style::default().fg(theme.mem_color))
            .data(&data);

        let chart = Chart::new(vec![dataset])
            .x_axis(
                Axis::default()
                    .bounds([0.0, app.mem_history.len() as f64])
                    .style(Style::default().fg(theme.border)),
            )
            .y_axis(
                Axis::default()
                    .bounds([0.0, 100.0])
                    .style(Style::default().fg(theme.border)),
            )
            .style(Style::default().bg(theme.bg_secondary));

        frame.render_widget(chart, chunks[4]);
    }
}
