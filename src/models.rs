
use colored::Colorize;
use serde::{Deserialize, Serialize};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
use toml::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub assignee: String,
    pub status: String,
}

impl Ticket {
    pub fn new(id: String, title: String, assignee: String, status: String) -> Ticket {
        Ticket {
            id,
            title,
            assignee,
            status,
        }
    }

    pub fn as_pretty_string(&self) -> String {
        format!(
            "[{:5}] {} / {} / {}",
            self.id.blue().underline(),
            self.title.green(),
            self.assignee.italic(),
            self.status.yellow()
        )
    }
}

pub struct Line<'a> {
    pub contents: &'a String,
    pub ticket: Option<&'a str>
}

impl <'a> Line<'a> {
    pub fn new (contents: &'a String) -> Line {
        Line { contents, ticket: extract_ticket_id(contents) }
    }
}

fn extract_ticket_id(line: &str) -> Option<&str> {
    let re = Regex::new(r"\b[A-Z]+-[0-9]+\b").unwrap();

    re.find(line).map(|mat| mat.as_str())
}

#[derive(Debug)]
pub struct JiraConfig {
    pub username: String,
    pub api_token: String,
    pub site: String,
}

impl JiraConfig {
    pub fn from_config() -> JiraConfig {
        // Get the home directory path
        let home_dir = env::var("HOME")
            .expect("HOME environment variable not found");

        // Construct the config file path
        let file_path = format!("{}/.jeba/conf.toml", home_dir);

        // Read the file contents into a String
        let mut file_content = String::new();
        File::open(file_path)
            .expect("Config file not found")
            .read_to_string(&mut file_content)
            .expect("Error reading config file");

        // Parse the TOML content into a Value
        let toml_value: Value = toml::from_str(&file_content)
            .expect("Error parsing TOML");

        // Extract values from the TOML structure
        let jira_section = toml_value["jira"].as_table().expect("[jira] section not found in config file");

        // Extract username, api_token, and site fields
        let username = jira_section["username"].as_str().expect("username not found in config").to_string();
        let api_token = jira_section["api_token"].as_str().expect("api_token not found in config").to_string();
        let site = jira_section["site"].as_str().expect("site not found in config").to_string();

        // Create and return the JiraConfig struct
        JiraConfig { username, api_token, site }
    }
}

