
use std::io;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;


use crate::{models::{Ticket, Line, JiraConfig}, jira::query_jira};

pub fn read_stdin() -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    break;
                } else {
                    lines.push(input)
                }
            }
            Err(error) => {
                eprintln!("error: {}", error);
            }
        }
    }
    lines
}

pub fn process_lines(lines: &[String]) {
    let structs: Vec<Line> = lines.iter().map(Line::new ).collect();

    let max_line_size = lines.iter().map(|l| l.len()).max().unwrap_or(10);

    let desired: Vec<&str> = structs.iter().filter_map(|line| line.ticket).collect();

    let all_tickets: HashMap<String, Ticket> = find_tickets(desired);

    for ele in structs {

        let printable = ele.ticket
            .and_then(|id| all_tickets.get(id))
            .map(|ticket| ticket.as_pretty_string())
            .unwrap_or_default();

        println!("{:<width$}\t{}", ele.contents.replace('\n', ""), printable, width = max_line_size);
    }
}

fn find_tickets(desired: Vec<&str>) -> HashMap<String, Ticket> {
    log::debug!("Total Number of entries requested: {}", desired.len());

    let cached = read_cache();

    log::debug!("Number of entries cached: {}", cached.len());

    let delta =  desired.into_iter().filter(|x| !cached.contains_key(*x) ).collect::<Vec<&str>>().join(", ");

    if delta.is_empty() {
        return cached
    }

    let config = JiraConfig::from_config();

    let found = query_jira(&delta, &config);

    let all: HashMap<String, Ticket>= cached.into_iter().chain(found).collect();

    let values: Vec<&Ticket> = all.values().collect();

    write_cache(values) ;

    all
}


fn read_cache() -> HashMap<String, Ticket> {
    let file_path = cache_location();

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return HashMap::new(),
    };

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Unable to read file");

    let tickets: Vec<Ticket> = serde_json::from_str(&file_content)
        .expect("Unable to deserialize - try deleting cache.json");
    
    tickets.into_iter().map(|ticket| (ticket.id.clone(), ticket)).collect()
}

fn write_cache(entries: Vec<&Ticket>) {
    let serialized_data = serde_json::to_string(&entries)
        .expect("Serialization failed");

    let file_path = cache_location();

    let mut file = File::create(file_path)
        .expect("Failed to create cache file");

    file.write_all(serialized_data.as_bytes())
        .expect("Write to cache failed");

    log::debug!("Cache entries written");

}

fn cache_location() -> String {
    let home_dir = env::var("HOME")
        .expect("Home directory must exist");

    let file_path = format!("{}/.jeba/cache.json", home_dir);
    file_path
}