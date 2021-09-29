use clap::{AppSettings, Clap};

use crate::Command;

use std::io::Read;
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::StatusCode;
use std::process::exit;
use std::io::stdin;

/// Interact with the Confluent Schema Registry
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Schemas {
    /// Schema Registry URL
    #[clap(short, long)]
    url: String,

    /// List subjects
    #[clap(short, long)]
    list: bool,

    /// Subject
    #[clap(short, long)]
    subject: Option<String>,

    /// Subject version
    #[clap(short, long)]
    version: Option<u32>,

    /// Delete a subject or subject version
    #[clap(short, long)]
    delete: bool,

    /// Return the unescaped schema
    #[clap(long)]
    schema: bool,

    /// Register a new schema
    #[clap(short, long)]
    register: bool,

    /// Check a schema
    #[clap(short, long)]
    check: bool,

    /// Escaped schema (or pipe the unescaped schema in)
    escaped_schema: Option<String>,
}

impl Command for Schemas {
    fn run(&self) {
        match self {
            Schemas {
                url,
                list: true,
                subject: None,
                version: None,
                delete: false,
                schema: false,
                register: false,
                check: false,
                escaped_schema: None,
            } => list(url),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: None,
                delete: false,
                schema: false,
                register: false,
                check: false,
                escaped_schema: None,
            } => get(url, subject),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: Some(version),
                delete: false,
                schema,
                register: false,
                check: false,
                escaped_schema: None,
            } => get_version(url, subject, version, schema),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: Some(version),
                delete: true,
                schema: false,
                register: false,
                check: false,
                escaped_schema: None,
            } => delete_version(url, subject, version),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: Some(version),
                delete: false,
                schema: false,
                register: false,
                check: true,
                escaped_schema: Some(escaped_schema),
            } => check_version(url, subject, version, escaped_schema),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: Some(version),
                delete: false,
                schema: false,
                register: false,
                check: true,
                escaped_schema: None,
            } => check_version_stdin(url, subject, version),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: None,
                delete: true,
                schema: false,
                register: false,
                check: false,
                escaped_schema: None,
            } => delete(url, subject),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: None,
                delete: false,
                schema: false,
                register: false,
                check: true,
                escaped_schema: None,
            } => check_stdin(url, subject),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: None,
                delete: false,
                schema: false,
                register: false,
                check: true,
                escaped_schema: Some(escaped_schema),
            } => check(url, subject, escaped_schema),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: None,
                delete: false,
                schema: false,
                register: true,
                check: false,
                escaped_schema: None,
            } => register_stdin(url, subject),
            Schemas {
                url,
                list: false,
                subject: Some(subject),
                version: None,
                delete: false,
                schema: false,
                register: true,
                check: false,
                escaped_schema: Some(escaped_schema),
            } => register(url, subject, escaped_schema),
            _ => {
                eprintln!("Unknown argument combination!");
                exit(1);
            }
        }
    }
}

fn list(url: &String) {
    //TODO ?deleted
    http_get(&format!("{}/subjects", url));
}

fn delete_version(url: &String, subject: &String, version: &u32) {
    http_get(&format!("{}/subjects/{}/versions/{}", url, subject, version));
}

fn check_version_stdin(url: &String, subject: &String, version: &u32) {
    //TODO ?verbose
    //TODO restructure input {"schema": "escaped_schema"}
    http_post(&format!("{}/compatibility/subjects/{}/versions/{}", url, subject, version), &piped_stdin().unwrap());
}

fn check_version(url: &String, subject: &String, version: &u32, escaped_schema: &String) {
    //TODO ?verbose
    //TODO restructure input {"schema": "escaped_schema"}
    http_post(&format!("{}/compatibility/subjects/{}/versions/{}", url, subject, version), escaped_schema);
}

fn get_version(url: &String, subject: &String, version: &u32, schema: &bool) {
    if *schema {
        http_get(&format!("{}/subjects/{}/versions/{}/schema", url, subject, version));
    } else {
        http_get(&format!("{}/subjects/{}/versions/{}", url, subject, version));
    }
}

fn delete(url: &String, subject: &String) {
    //TODO delete ?permanent
    http_delete(&format!("{}/subjects/{}", url, subject));
}

fn check_stdin(url: &String, subject: &String) {
    //TODO restructure input {"schema": "escaped_schema"}
    http_post(&format!("{}/subjects/{}", url, subject), &piped_stdin().unwrap());
}

fn check(url: &String, subject: &String, escaped_schema: &String) {
    //TODO restructure input {"schema": "escaped_schema"}
    http_post(&format!("{}/subjects/{}", url, subject), escaped_schema);
}

fn get(url: &String, subject: &String) {
    http_get(&format!("{}/subjects/{}/versions", url, subject));
}

fn register_stdin(url: &String, subject: &String) {
    //TODO restructure input {"schema": "escaped_schema"}
    http_post(&format!("{}/subjects/{}/versions", url, subject), &piped_stdin().unwrap());
}

fn register(url: &String, subject: &String, escaped_schema: &String) {
    //TODO restructure input {"schema": "escaped_schema"}
    http_post(&format!("{}/subjects/{}/versions", url, subject), escaped_schema);
}

fn piped_stdin() -> Result<String, String> {
    if !atty::is(atty::Stream::Stdin) {
        println!("stdin");
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)
            .expect("Failed stdin");
        Ok(buffer)
    } else {
        Err("No piped input found".to_string())
    }
}

fn http_get(url: &String) {
    http(|client: Client| client.get(url));
}

fn http_post(url: &String, body: &String) {
    http(|client: Client| client.post(url).body(body.clone()));
}

fn http_delete(url: &String) {
    http(|client: Client| client.delete(url));
}

fn http<F>(request: F) where F: Fn(Client) -> RequestBuilder {
    let client = Client::new();
    let response = request(client).send().unwrap();
    let status = response.status();
    let body = response.text().unwrap();
    match status {
        StatusCode::OK => {
            println!("{}", body);
            exit(0);
        }
        _ => {
            eprintln!("{}", body);
            exit(1);
        }
    }
}
