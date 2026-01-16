mod app;
mod data;
mod ui;

use std::io;
use std::time::Duration;

use app::{App, KillSignal};
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// A beautiful terminal-based system monitor
#[derive(Parser, Debug)]
#[command(name = "mprobe")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Update interval in milliseconds
    #[arg(short = 'i', long, default_value = "250", value_name = "MS")]
    update_interval: u64,

    /// Disable colors (use default terminal colors)
    #[arg(long)]
    no_color: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    app.no_color = args.no_color;
    let res = run_app(&mut terminal, &mut app, args.update_interval);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {err:?}");
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    update_interval: u64,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(update_interval);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Handle help screen first
                if app.show_help {
                    match key.code {
                        KeyCode::Char('?') | KeyCode::Esc | KeyCode::Char('q') => {
                            app.show_help = false;
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle kill confirmation mode
                if app.kill_confirm.is_some() {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => app.confirm_kill(),
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => app.cancel_kill(),
                        _ => {}
                    }
                    continue;
                }

                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(())
                    }
                    KeyCode::Char('?') => app.toggle_help(),
                    KeyCode::Tab => app.next_tab(),
                    KeyCode::BackTab => app.prev_tab(),
                    KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
                    KeyCode::Char('g') => app.scroll_to_top(),
                    KeyCode::Char('G') => app.scroll_to_bottom(),
                    KeyCode::Char('/') => app.toggle_filter_mode(),
                    KeyCode::Char('t') => app.toggle_tree_view(),
                    KeyCode::Char('s') => app.cycle_sort(),
                    KeyCode::Char('r') => app.toggle_sort_order(),
                    KeyCode::Char('x') => app.initiate_kill(KillSignal::Term),
                    KeyCode::Char('X') => app.initiate_kill(KillSignal::Kill),
                    KeyCode::Esc => app.clear_filter(),
                    KeyCode::Char(c) if app.filter_mode => app.add_filter_char(c),
                    KeyCode::Backspace if app.filter_mode => app.remove_filter_char(),
                    KeyCode::Enter if app.filter_mode => app.toggle_filter_mode(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            app.clear_expired_status();
            last_tick = std::time::Instant::now();
        }
    }
}
