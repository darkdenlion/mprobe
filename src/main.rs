mod app;
mod config;
mod data;
mod ui;

use std::io;
use std::time::Duration;

use app::{App, KillSignal};
use clap::Parser;
use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers, MouseEventKind},
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
    #[arg(short = 'i', long, value_name = "MS")]
    update_interval: Option<u64>,

    /// Disable colors (use default terminal colors)
    #[arg(long)]
    no_color: bool,

    /// Generate default config file at ~/.config/mprobe/config.toml
    #[arg(long)]
    generate_config: bool,

    /// Show config file path
    #[arg(long)]
    config_path: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Handle --config-path flag
    if args.config_path {
        match Config::config_path() {
            Some(path) => println!("{}", path.display()),
            None => eprintln!("Could not determine config directory"),
        }
        return Ok(());
    }

    // Handle --generate-config flag
    if args.generate_config {
        match Config::config_path() {
            Some(path) => {
                let config = Config::default();
                match config.save() {
                    Ok(()) => println!("Config file created at: {}", path.display()),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            None => eprintln!("Could not determine config directory"),
        }
        return Ok(());
    }

    // Load config from file
    let config = Config::load();

    // CLI args override config file
    let update_interval = args.update_interval.unwrap_or(config.update_interval);
    let no_color = args.no_color || config.no_color;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run
    let mut app = App::new();
    app.no_color = no_color;
    app.apply_config(&config);
    let res = run_app(&mut terminal, &mut app, update_interval);

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
            match event::read()? {
                Event::Key(key) => {
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
                        KeyCode::Char('c') => app.toggle_connections(),
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
                Event::Mouse(mouse) => {
                    // Close dialogs on any click
                    if app.show_help || app.kill_confirm.is_some() {
                        if matches!(mouse.kind, MouseEventKind::Down(_)) {
                            app.show_help = false;
                            app.cancel_kill();
                        }
                        continue;
                    }

                    match mouse.kind {
                        MouseEventKind::ScrollUp => {
                            // Scroll up 3 lines at a time for faster scrolling
                            for _ in 0..3 {
                                app.scroll_up();
                            }
                        }
                        MouseEventKind::ScrollDown => {
                            // Scroll down 3 lines at a time for faster scrolling
                            for _ in 0..3 {
                                app.scroll_down();
                            }
                        }
                        MouseEventKind::Down(_) => {
                            // Click to select process in the process list area
                            // Process list starts after header (5) + top row (11) + bottom row (9) = 25 lines
                            // Plus 1 for the table header row
                            let process_area_start = 26u16;
                            if mouse.row >= process_area_start {
                                let clicked_row = (mouse.row - process_area_start) as usize;
                                if clicked_row < app.process_data.processes.len() {
                                    app.process_scroll = clicked_row;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            app.clear_expired_status();
            last_tick = std::time::Instant::now();
        }
    }
}
