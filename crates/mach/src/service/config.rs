use crate::entity::config;
use miette::IntoDiagnostic;
use sea_orm::{ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeekStart {
    Sunday,
    Monday,
}

impl WeekStart {
    pub fn toggle(self) -> Self {
        match self {
            WeekStart::Sunday => WeekStart::Monday,
            WeekStart::Monday => WeekStart::Sunday,
        }
    }
}

impl From<&str> for WeekStart {
    fn from(value: &str) -> Self {
        match value {
            "monday" => WeekStart::Monday,
            _ => WeekStart::Sunday,
        }
    }
}

impl WeekStart {
    pub fn as_str(&self) -> &'static str {
        match self {
            WeekStart::Sunday => "sunday",
            WeekStart::Monday => "monday",
        }
    }
}

#[derive(Clone)]
pub struct ConfigService {
    db: DatabaseConnection,
}

impl ConfigService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn load_week_start(&self) -> miette::Result<WeekStart> {
        let result = config::Entity::find()
            .filter(config::Column::Key.eq("week_start"))
            .one(&self.db)
            .await
            .into_diagnostic()?;

        if let Some(model) = result {
            if let Some(value) = model.value.as_str() {
                return Ok(WeekStart::from(value));
            }
        }

        Ok(WeekStart::Sunday)
    }

    pub async fn save_week_start(&self, week_start: WeekStart) -> miette::Result<()> {
        let model = config::ActiveModel {
            key: Set("week_start".to_string()),
            value: Set(json!(week_start.as_str())),
            ..Default::default()
        };

        config::Entity::insert(model)
            .on_conflict(
                sea_orm::sea_query::OnConflict::column(config::Column::Key)
                    .update_column(config::Column::Value)
                    .to_owned(),
            )
            .exec(&self.db)
            .await
            .into_diagnostic()?;

        Ok(())
    }
}
