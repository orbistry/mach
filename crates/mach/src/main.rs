#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = mach::Cli::default();

    cli.exec().await
}
