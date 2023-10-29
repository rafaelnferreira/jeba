use crate::services::read_stdin;
use crate::services::process_lines;

use log::LevelFilter;
use structopt::StructOpt;


mod services;
mod models;
mod jira;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(long = "debug")]
    debug: bool,
}

fn main() {

    let args = Cli::from_args();

    // Determine log level based on the --debug flag
    let log_level = if args.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    // Initialize the logger with the determined log level
    env_logger::Builder::from_default_env()
        .filter_level(log_level)
        .init();


    let lines = read_stdin(); 

    process_lines(&lines);

}
