mod processes;
mod state;
mod to_do;
use chrono::Local;
use processes::process_input;
use serde_json::value::Value;
use serde_json::Map;
use state::read_file;
use std::env;
use to_do::to_do_factory;

pub const FILE: &str = "state.json";

fn main() {
    let last_update: String = format!("{}", Local::now().format("%e/%b/%Y-%H:%M:%S"));

    let args: Vec<String> = env::args().collect();

    let command: &String = &args[1];
    let mut title: &String = &String::from("");

    if args.len() >= 3 {
        title = &args[2]
    }

    let state: Map<String, Value> = read_file(FILE);

    let status: String;

    match &state.get(*&title) {
        Some(result) => status = result.get("status").unwrap().to_string().replace('\"', ""),
        None => status = String::from("pending"),
    }
    let item = to_do_factory(&status, title, &last_update).expect(&status);
    process_input(item, command.to_string(), &state);
}
