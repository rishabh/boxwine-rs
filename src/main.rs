use anyhow::Result;
use clap::Clap;

mod config;
mod create;
mod files;
mod init;

/// Box up your Wine apps and turn them into Mac Apps.
#[derive(Clap)]
#[clap(version = clap::crate_version!())]
struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    Create(create::Create),
    Init(init::Init),
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    // Dispatch handlers for subcommands
    match opts.subcmd {
        SubCommand::Create(create_opts) => create::create(create_opts),
        SubCommand::Init(init_opts) => init::init(init_opts),
    }
}
