pub mod add;
pub mod list;

#[derive(clap::Subcommand)]
pub enum Cmd {
    Add(add::Args),
    List(list::Args),
}

impl Cmd {
    pub async fn exec(self, services: &crate::service::Services) -> miette::Result<()> {
        match self {
            Cmd::Add(args) => args.exec(services).await,
            Cmd::List(args) => args.exec(services).await,
        }
    }
}
