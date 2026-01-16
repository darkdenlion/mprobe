use crate::app::App;
use crate::ui::Theme;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App, area: Rect, theme: &Theme) {
    let listening_count = app.connection_data.listening_ports.len();
    let established_count = app.connection_data.connections.len();

    let block = Block::default()
        .title(Line::from(vec![
            Span::styled(" ", Style::default()),
            Span::styled(
                "CONNECTIONS",
                Style::default()
                    .fg(theme.net_down)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" ({}↓ {}●) ", established_count, listening_count),
                Style::default().fg(theme.fg_muted),
            ),
        ]))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.bg_secondary));

    // Header
    let header_cells = ["PROTO", "LOCAL", "REMOTE", "STATE"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(theme.table_header)));

    let header = Row::new(header_cells)
        .style(Style::default().bg(theme.bg_secondary))
        .height(1);

    // Combine listening ports and connections
    let mut all_connections: Vec<_> = app.connection_data.listening_ports.iter()
        .chain(app.connection_data.connections.iter())
        .take(50) // Limit to avoid too many rows
        .collect();

    // Sort: listening first, then by state
    all_connections.sort_by(|a, b| {
        let a_listen = a.state == "LISTEN";
        let b_listen = b.state == "LISTEN";
        b_listen.cmp(&a_listen)
    });

    let rows = all_connections.iter().map(|conn| {
        let state_color = match conn.state.as_str() {
            "LISTEN" => theme.success,
            "ESTABLISHED" | "ESTAB" => theme.accent,
            "TIME_WAIT" | "TIME-WAIT" => theme.fg_muted,
            "CLOSE_WAIT" | "CLOSE-WAIT" => theme.warning,
            _ => theme.fg_dim,
        };

        let cells = vec![
            Cell::from(conn.protocol.clone()).style(Style::default().fg(theme.fg_dim)),
            Cell::from(truncate_addr(&conn.local_addr, 20)).style(Style::default().fg(theme.fg)),
            Cell::from(truncate_addr(&conn.remote_addr, 20)).style(Style::default().fg(theme.fg_dim)),
            Cell::from(format_state(&conn.state)).style(Style::default().fg(state_color)),
        ];

        Row::new(cells)
    });

    let widths = [
        Constraint::Length(6),   // Protocol
        Constraint::Min(15),     // Local
        Constraint::Min(15),     // Remote
        Constraint::Length(12),  // State
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .style(Style::default().bg(theme.bg_secondary));

    frame.render_widget(table, area);
}

fn truncate_addr(addr: &str, max_len: usize) -> String {
    if addr.len() > max_len {
        format!("{}…", &addr[..max_len - 1])
    } else {
        addr.to_string()
    }
}

fn format_state(state: &str) -> String {
    match state {
        "ESTABLISHED" => "ESTAB".to_string(),
        "CLOSE_WAIT" => "CLOSE-W".to_string(),
        "TIME_WAIT" => "TIME-W".to_string(),
        "CLOSE-WAIT" => "CLOSE-W".to_string(),
        "TIME-WAIT" => "TIME-W".to_string(),
        _ => state.chars().take(10).collect(),
    }
}
