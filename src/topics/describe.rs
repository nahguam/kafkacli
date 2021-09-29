use clap::{AppSettings, Clap};
use serde::Serialize;
use kafka::client::KafkaClient;
use crate::Command;

/// List
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Describe {
    /// Kafka bootstrap servers
    #[clap(short, long)]
    bootstrap_servers: String,

    /// Topic
    #[clap(short, long)]
    topic: String,
}

impl Command for Describe {
    fn run(&self) {
        let mut client = KafkaClient::new(vec![self.bootstrap_servers.to_string()]);

        client.load_metadata(&[&self.topic]).unwrap();

        let topics = client.topics();
        let partitions: Vec<Partition> = topics.partitions(&self.topic).unwrap().iter()
            .map(|p| Partition {
                id: p.id(),
                is_available: p.is_available(),
                leader: p.leader().map(|l| Broker {
                    id: l.id(),
                    host: l.host().to_string(),
                }).unwrap(),
            }).collect();

        let topic = Topic {
            name: self.topic.clone(),
            partitions,
        };

        println!("{}", serde_json::to_string_pretty(&topic)
            .expect("Unable to serialise"));
    }
}

#[derive(Serialize)]
struct Topic {
    name: String,
    partitions: Vec<Partition>,
}

#[derive(Serialize)]
struct Partition {
    id: i32,
    is_available: bool,
    leader: Broker,
}

#[derive(Serialize)]
struct Broker {
    id: i32,
    host: String,
}
