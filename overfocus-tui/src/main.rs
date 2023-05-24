use std::time::Duration;

use app::App;
use crossterm::{event::{EnableMouseCapture, DisableMouseCapture}, terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute};
use tui::{backend::{CrosstermBackend, Backend}, Terminal};

type BackendTerminal = Terminal<CrosstermBackend<std::io::Stdout>>;

mod app;

fn main() {
    let terminal = setup_terminal().unwrap();

    // Run application
    let mut app = App::new(terminal);
    app.run(Duration::from_millis(200));

    terminate_terminal(app).unwrap();
}

fn setup_terminal() -> anyhow::Result<BackendTerminal> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn terminate_terminal<B: Backend + std::io::Write>(mut app: App<B>) -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(app.terminal_mut().backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    app.terminal_mut().show_cursor()?;
    Ok(())
}