/// List all todos in a table
#[derive(clap::Args)]
pub struct Args {
    /// List todos in the backlog
    #[clap(short, long, default_value = "false")]
    some_day: bool,

    /// Include completed todos
    #[clap(short, long, default_value = "false")]
    done: bool,
}

impl Args {
    pub async fn exec(self) -> miette::Result<()> {
        Ok(())
    }
}
