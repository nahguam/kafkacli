use clap::{AppSettings, Clap};

use list::List;
use describe::Describe;
use crate::Command;

mod describe;
mod list;

/// Topics
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Topics {
    #[clap(subcommand)]
    sub_command: SubCommand,
}

trait_enum! {
    #[derive(Clap, Debug)]
    enum SubCommand : Command {
        List,
        Describe,
    }
}

impl Command for Topics {
    fn run(&self) {
        self.sub_command.run()
    }
}
