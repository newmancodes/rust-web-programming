mod state;

use std::env;
use state::{write_to_file, read_file};
use serde_json::value::Value;
use serde_json::{Map, json};

fn main() {
    let args: Vec<String> = env::args().collect();
    let status: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = read_file(&String::from("./state.json"));
    println!("{:?}", state);

    state.insert(title.to_string(), json!(status));
    write_to_file("./state.json", &mut state);
}
