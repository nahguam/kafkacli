use clap::{AppSettings, Clap};

use kafka::client::KafkaClient;
use crate::Command;

/// List
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct List {
    /// Kafka bootstrap servers
    #[clap(short, long)]
    bootstrap_servers: String,
}

impl Command for List {
    fn run(&self) {
        let mut client = KafkaClient::new(vec![self.bootstrap_servers.to_string()]);

        client.load_metadata_all().unwrap();

        let topics = client.topics();
        let names = topics.names()
            .collect::<Vec<&str>>();
        let json = serde_json::to_string_pretty(&names)
            .expect("Unable to serialise");

        println!("{}", json);
    }
}
