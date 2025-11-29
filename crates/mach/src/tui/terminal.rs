use std::io;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use miette::{Context, IntoDiagnostic};
use ratatui::{Terminal, backend::CrosstermBackend};

pub fn setup_terminal() -> miette::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()
        .into_diagnostic()
        .wrap_err("failed to enable raw mode")?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)
        .into_diagnostic()
        .wrap_err("failed to enter alternate screen")?;

    let backend = CrosstermBackend::new(stdout);

    Terminal::new(backend)
        .into_diagnostic()
        .wrap_err("failed to initialize terminal")
}

pub struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();

        let mut stdout = io::stdout();

        let _ = execute!(stdout, LeaveAlternateScreen);
    }
}
