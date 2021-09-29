use clap::{AppSettings, Clap};

// use kafka::consumer::Consumer;
use crate::Command;

/// Consume from Kafka
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Consume {
    /// Kafka bootstrap servers
    #[clap(short, long)]
    bootstrap_servers: String,
}

impl Command for Consume {
    fn run(&self) {
        println!("consume");
    }
}
