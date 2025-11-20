use crate::service::Services;

/// Add a new todo
#[derive(clap::Args)]
pub struct Args {
    /// Insert the todo into the backlog
    #[clap(short, long, default_value = "false")]
    some_day: bool,

    /// Title of the todo (quoted or space separated)
    #[clap(required = true)]
    title: Vec<String>,
}

impl Args {
    pub async fn exec(self, services: &Services) -> miette::Result<()> {
        let scheduled_for = if self.some_day {
            None
        } else {
            Some(services.today())
        };

        let todo = services
            .todos
            .add(self.title(), scheduled_for, None)
            .await?;

        let date_label = scheduled_for
            .map(|d| d.to_string())
            .unwrap_or_else(|| "Someday".into());

        println!("Added todo '{}' -> {}", todo.title, date_label);

        Ok(())
    }

    fn title(&self) -> String {
        self.title.join(" ")
    }
}
