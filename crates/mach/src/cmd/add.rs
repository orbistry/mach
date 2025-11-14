/// Add a new todo
#[derive(clap::Args)]
pub struct Args {
    /// Insert the todo into the backlog
    #[clap(short, long, default_value = "false")]
    some_day: bool,
}

impl Args {
    pub async fn exec(self) -> miette::Result<()> {
        Ok(())
    }
}
