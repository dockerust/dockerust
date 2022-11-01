use clap::{Parser, Subcommand};
use dockerust::commands::init::Init;
use dockerust::commands::run::Run;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Create a container with namespace and cgroups limit dockerust run -ti [command]
    Run(Run),
    /// Init container process run user's process in container. Internal command, don't call it outside.
    #[command(hide = true)]
    Init(Init),
}

fn main() {
    let cli: Cli = Cli::parse();
    tracing_subscriber::fmt::init();

    match cli.subcmd {
        SubCommand::Run(run) => run.exec(),
        SubCommand::Init(init) => init.exec(),
    };
}
