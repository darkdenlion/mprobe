use crate::app::App;
use crate::ui::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let usage_color = theme.get_usage_color(app.cpu_data.total_usage);

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(
                "CPU",
                Style::default()
                    .fg(theme.cpu_color)
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
            Constraint::Length(1), // Stats line
            Constraint::Min(3),    // Graph
            Constraint::Length(2), // Progress bar
        ])
        .split(inner);

    // Stats line
    let stats = Line::from(vec![
        Span::styled(
            format!("{:.1}%", app.cpu_data.total_usage),
            Style::default()
                .fg(usage_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("{} cores", app.cpu_data.core_count),
            Style::default().fg(theme.fg_dim),
        ),
        Span::styled("  ", Style::default()),
        Span::styled(
            format!("{} MHz", app.cpu_data.frequency),
            Style::default().fg(theme.fg_muted),
        ),
    ]);
    frame.render_widget(
        ratatui::widgets::Paragraph::new(stats),
        chunks[0],
    );

    // Graph
    let data: Vec<(f64, f64)> = app
        .cpu_history
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f64, v))
        .collect();

    let dataset = Dataset::default()
        .marker(Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(theme.cpu_color))
        .data(&data);

    let chart = Chart::new(vec![dataset])
        .x_axis(
            Axis::default()
                .bounds([0.0, app.cpu_history.len() as f64])
                .style(Style::default().fg(theme.border)),
        )
        .y_axis(
            Axis::default()
                .bounds([0.0, 100.0])
                .style(Style::default().fg(theme.border)),
        )
        .style(Style::default().bg(theme.bg_secondary));

    frame.render_widget(chart, chunks[1]);

    // Progress bar representation
    let bar_width = chunks[2].width.saturating_sub(2) as usize;
    let filled = ((app.cpu_data.total_usage / 100.0) * bar_width as f64) as usize;
    let empty = bar_width.saturating_sub(filled);

    let bar = Line::from(vec![
        Span::styled(
            "▓".repeat(filled),
            Style::default().fg(usage_color),
        ),
        Span::styled(
            "░".repeat(empty),
            Style::default().fg(theme.border),
        ),
    ]);
    frame.render_widget(
        ratatui::widgets::Paragraph::new(bar),
        chunks[2],
    );
}
