use std::time::{Duration, Instant};

use crossterm::event;
use miette::{Context, IntoDiagnostic};
use tokio::runtime::Handle;

use crate::service::{Services, config::WeekStart};

mod actions;
mod cursor;
mod draw;
mod input;
mod modes;
pub mod palette;
mod state;
mod terminal;

use cursor::{BacklogCursor, CursorState};
use modes::UiMode;
use state::{BoardData, WeekState};
use terminal::{TerminalGuard, setup_terminal};

/// Launch the Ratatui application, blocking on the UI event loop.
pub async fn run(services: Services) -> miette::Result<()> {
    let handle = Handle::current();
    tokio::task::spawn_blocking(move || {
        let mut app = App::new(services, handle);
        app.run()
    })
    .await
    .into_diagnostic()??;
    Ok(())
}

pub struct App {
    services: Services,
    runtime: Handle,
    state: WeekState,
    board: BoardData,
    cursor: CursorState,
    backlog_cursor: BacklogCursor,
    week_pref: WeekStart,
    ui_mode: UiMode,
    pending_g: bool,
    pending_delete: bool,
    should_quit: bool,
    show_help: bool,
}

impl App {
    fn new(services: Services, runtime: Handle) -> Self {
        let today = services.today();
        let week_pref = services.week_start();
        let state = WeekState::new(today, week_pref);
        let board = BoardData::new(state.columns.len());
        let mut cursor = CursorState::new(state.columns.len());
        if let Some(idx) = state.column_index(today) {
            cursor.set_focus_row(idx, 0);
        }
        Self {
            services,
            runtime,
            state,
            board,
            cursor,
            backlog_cursor: BacklogCursor::new(),
            week_pref,
            ui_mode: UiMode::Board,
            pending_g: false,
            pending_delete: false,
            should_quit: false,
            show_help: false,
        }
    }

    fn run(&mut self) -> miette::Result<()> {
        self.refresh_board().ok();

        let mut terminal = setup_terminal()?;
        let _guard = TerminalGuard;

        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(250);

        loop {
            terminal
                .draw(|frame| self.draw(frame))
                .into_diagnostic()
                .wrap_err("failed to draw frame")?;

            if self.should_quit {
                break;
            }

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).into_diagnostic()? {
                let evt = event::read().into_diagnostic()?;
                self.handle_event(evt);
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        Ok(())
    }
}
