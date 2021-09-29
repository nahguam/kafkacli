use clap::{AppSettings, Clap};

use schemas::Schemas;
use consume::Consume;
use topics::Topics;

mod consume;
mod topics;
mod schemas;


#[macro_use]
extern crate trait_enum;

/// Opts
#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    sub_command: SubCommand,
}

trait_enum! {
    #[derive(Clap, Debug)]
    enum SubCommand : Command {
        Topics,
        Consume,
        Schemas,
    }
}

trait Command {
    fn run(&self);
}

fn main() {
    let opts = Opts::parse();
    opts.sub_command.run();
}
