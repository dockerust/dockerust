use clap::{Parser, Subcommand};
use dockerust::commands::{create, delete, init, kill, query, run, start};
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
    Query(query::Query),
    Create(create::Create),
    Start(start::Start),
    Kill(kill::Kill),
    Delete(delete::Delete),
    /// Create a container with namespace and cgroups limit dockerust run -t [image] [commands]
    Run(run::Run),
    /// Init container process run user's process in container. Internal command, don't call it outside.
    #[command(hide = true)]
    Init(init::Init),
}

fn main() {
    let cli: Cli = Cli::parse();
    tracing_subscriber::fmt::init();

    unsafe {
        match cli.subcmd {
            SubCommand::Query(query) => query.exec(),
            SubCommand::Create(create) => create.exec(),
            SubCommand::Start(start) => start.exec(),
            SubCommand::Kill(kill) => kill.exec(),
            SubCommand::Delete(delete) => delete.exec(),
            SubCommand::Run(run) => run.exec(),
            SubCommand::Init(init) => init.exec(),
        };
    }
}
