use chrono::{Datelike, Duration as ChronoDuration, NaiveDate};
use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use uuid::Uuid;

use crate::entity::todo;
use crate::service::config::WeekStart;

use super::palette;

pub const BACKLOG_COLUMNS: usize = 4;

pub struct WeekState {
    pub week_start: NaiveDate,
    pub columns: Vec<ColumnMeta>,
}

impl WeekState {
    pub fn new(today: NaiveDate, preference: WeekStart) -> Self {
        let week_start = start_of_week(today, preference);
        Self {
            week_start,
            columns: build_columns(week_start),
        }
    }

    pub fn prev_week(&mut self) {
        self.week_start -= ChronoDuration::days(7);
        self.columns = build_columns(self.week_start);
    }

    pub fn next_week(&mut self) {
        self.week_start += ChronoDuration::days(7);
        self.columns = build_columns(self.week_start);
    }

    pub fn column_index(&self, date: NaiveDate) -> Option<usize> {
        self.columns.iter().position(|col| col.date == date)
    }
}

#[derive(Clone)]
pub struct ColumnMeta {
    pub title: String,
    pub date: NaiveDate,
}

pub struct BoardData {
    pub days: Vec<Vec<TodoView>>,
    pub backlog_columns: [Vec<TodoView>; BACKLOG_COLUMNS],
}

impl BoardData {
    pub fn new(num_days: usize) -> Self {
        Self {
            days: vec![Vec::new(); num_days],
            backlog_columns: Default::default(),
        }
    }

    pub fn reset(&mut self, num_days: usize) {
        self.days = vec![Vec::new(); num_days];
        for col in &mut self.backlog_columns {
            col.clear();
        }
    }

    pub fn set_day(&mut self, idx: usize, todos: Vec<TodoView>) {
        if idx >= self.days.len() {
            self.days.resize(idx + 1, Vec::new());
        }
        self.days[idx] = todos;
    }

    pub fn day_len(&self, idx: usize) -> usize {
        self.days.get(idx).map(|d| d.len()).unwrap_or(0)
    }

    pub fn day_todo_id_at(&self, col: usize, row: usize) -> Option<Uuid> {
        self.days.get(col)?.get(row).map(|todo| todo.id)
    }

    pub fn set_backlog_column(&mut self, col: usize, todos: Vec<TodoView>) {
        if col < BACKLOG_COLUMNS {
            self.backlog_columns[col] = todos;
        }
    }

    pub fn backlog_col_len(&self, col: usize) -> usize {
        if col < BACKLOG_COLUMNS {
            self.backlog_columns[col].len()
        } else {
            0
        }
    }

    pub fn backlog_todo_id_at(&self, col: usize, row: usize) -> Option<Uuid> {
        self.backlog_columns.get(col)?.get(row).map(|todo| todo.id)
    }

    pub fn find_day_position(&self, id: Uuid) -> Option<(usize, usize)> {
        for (idx, day) in self.days.iter().enumerate() {
            if let Some(pos) = day.iter().position(|todo| todo.id == id) {
                return Some((idx, pos));
            }
        }
        None
    }

    pub fn find_backlog_position(&self, id: Uuid) -> Option<(usize, usize)> {
        for (col, items) in self.backlog_columns.iter().enumerate() {
            if let Some(pos) = items.iter().position(|todo| todo.id == id) {
                return Some((col, pos));
            }
        }
        None
    }

    pub fn day_status_of(&self, id: Uuid) -> Option<&str> {
        for day in &self.days {
            if let Some(todo) = day.iter().find(|todo| todo.id == id) {
                return Some(todo.status.as_str());
            }
        }
        None
    }

    pub fn backlog_status_of(&self, id: Uuid) -> Option<&str> {
        for col in &self.backlog_columns {
            if let Some(todo) = col.iter().find(|todo| todo.id == id) {
                return Some(todo.status.as_str());
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct TodoView {
    pub id: Uuid,
    pub title: String,
    pub status: String,
}

impl TodoView {
    pub fn to_line_with_prefix(&self, selected: bool) -> Line<'_> {
        let text = if selected {
            format!("› {}", self.title)
        } else {
            self.title.clone()
        };
        let mut line = Line::from(text);
        if self.status == "done" {
            line.style = Style::default()
                .fg(palette::TEXT_DIM)
                .add_modifier(Modifier::CROSSED_OUT | Modifier::DIM);
        } else {
            line.style = Style::default().fg(palette::TEXT);
        }
        line
    }
}

impl From<todo::Model> for TodoView {
    fn from(model: todo::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            status: model.status,
        }
    }
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
        cols.push(ColumnMeta { title, date });
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

pub fn start_of_week(date: NaiveDate, preference: WeekStart) -> NaiveDate {
    let weekday = date.weekday();
    let offset = match preference {
        WeekStart::Sunday => weekday.num_days_from_sunday() as i64,
        WeekStart::Monday => weekday.num_days_from_monday() as i64,
    };
    date - ChronoDuration::days(offset)
}
