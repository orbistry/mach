use std::{
    io,
    time::{Duration, Instant},
};

use chrono::{Datelike, Duration as ChronoDuration, NaiveDate};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use miette::{Context, IntoDiagnostic};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::service::Services;

/// Launch the Ratatui application, blocking on the UI event loop.
pub async fn run(services: Services) -> miette::Result<()> {
    let tui_task = tokio::task::spawn_blocking(move || {
        let mut app = App::new(services);

        app.run()
    });

    tui_task.await.into_diagnostic()??;

    Ok(())
}

struct App {
    services: Services,
    state: WeekState,
    should_quit: bool,
}

impl App {
    fn new(services: Services) -> Self {
        let today = services.today();
        let state = WeekState::new(today);

        Self {
            services,
            state,
            should_quit: false,
        }
    }

    fn run(&mut self) -> miette::Result<()> {
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

    fn handle_event(&mut self, evt: Event) {
        if let Event::Key(key) = evt {
            self.handle_key_event(key);
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') if key.modifiers.is_empty() => self.should_quit = true,
            KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('h') => self.state.focus_prev(),
            KeyCode::Char('l') => self.state.focus_next(),
            KeyCode::Char('[') => self.state.prev_week(),
            KeyCode::Char(']') => self.state.next_week(),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true
            }
            _ => {}
        }
    }

    fn draw(&mut self, frame: &mut Frame<'_>) {
        let _ = &self.services;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(10)])
            .split(frame.area());

        let week_columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(self.state.day_constraints())
            .split(chunks[0]);

        for (idx, area) in week_columns.iter().enumerate() {
            self.draw_column(frame, idx, *area);
        }

        self.draw_backlog(frame, chunks[1]);
    }

    fn draw_column(&self, frame: &mut Frame<'_>, idx: usize, area: Rect) {
        let column = &self.state.columns[idx];
        let focused = idx == self.state.focused_column;
        let title = Line::from(column.title.clone());

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(if focused {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default()
            });

        let body = Paragraph::new("No tasks yet").block(block);
        frame.render_widget(body, area);
    }

    fn draw_backlog(&self, frame: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .title("Someday / Backlog")
            .borders(Borders::ALL)
            .border_style(Style::default());
        let body = Paragraph::new("Backlog todos render here").block(block);
        frame.render_widget(body, area);
    }
}

struct WeekState {
    week_start: NaiveDate,
    focused_column: usize,
    columns: Vec<ColumnMeta>,
}

impl WeekState {
    fn new(today: NaiveDate) -> Self {
        let week_start = start_of_week(today);
        Self {
            week_start,
            focused_column: 0,
            columns: build_columns(week_start),
        }
    }

    fn day_constraints(&self) -> Vec<Constraint> {
        let count = self.columns.len() as u32;
        self.columns
            .iter()
            .map(|_| Constraint::Ratio(1, count))
            .collect()
    }

    fn focus_next(&mut self) {
        self.focused_column = (self.focused_column + 1) % self.columns.len();
    }

    fn focus_prev(&mut self) {
        if self.focused_column == 0 {
            self.focused_column = self.columns.len() - 1;
        } else {
            self.focused_column -= 1;
        }
    }

    fn prev_week(&mut self) {
        self.week_start -= ChronoDuration::days(7);
        self.columns = build_columns(self.week_start);
    }

    fn next_week(&mut self) {
        self.week_start += ChronoDuration::days(7);
        self.columns = build_columns(self.week_start);
    }
}

#[derive(Clone)]
struct ColumnMeta {
    title: String,
}

fn build_columns(week_start: NaiveDate) -> Vec<ColumnMeta> {
    let mut cols = Vec::with_capacity(7);
    for offset in 0..7 {
        let date = week_start + ChronoDuration::days(offset);
        let title = format!(
            "{} {:02}/{:02}",
            weekday_label(date.weekday()),
            date.month(),
            date.day()
        );
        cols.push(ColumnMeta { title });
    }
    cols
}

fn weekday_label(day: chrono::Weekday) -> &'static str {
    match day {
        chrono::Weekday::Mon => "Mon",
        chrono::Weekday::Tue => "Tue",
        chrono::Weekday::Wed => "Wed",
        chrono::Weekday::Thu => "Thu",
        chrono::Weekday::Fri => "Fri",
        chrono::Weekday::Sat => "Sat",
        chrono::Weekday::Sun => "Sun",
    }
}

fn start_of_week(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().number_from_monday() as i64 - 1;
    date - ChronoDuration::days(weekday)
}

fn setup_terminal() -> miette::Result<Terminal<CrosstermBackend<io::Stdout>>> {
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

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
    }
}
