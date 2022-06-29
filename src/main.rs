use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::env;
use std::process;
use itertools::Itertools;
use std::path::Path;

mod model;
use crate::model::input::Payload;
use crate::model::input::Event;
use crate::model::patient_data::Patient;
use crate::model::patient_data::Case;

fn process_events(events: Vec<Event>) -> Result<HashMap<Patient, Vec<Case>>, &'static str> {
    let mut patient_list: HashMap<Patient, Vec<Case>> = HashMap::new();
    for event in events {
        match event.payload {
            Payload::Patient {
                patient_id,
                name
            } => {
                patient_list.entry(Patient::new(patient_id, name)).or_insert(Vec::new());
            }
            Payload::File {
                patient_id,
                file
            } => {
                patient_list.get_mut(&Patient::new(patient_id, String::from(" "))).unwrap().push(Case::new(file));
            }
        }
    }
    return Ok(patient_list);
}

fn print_events(processed_events: HashMap<Patient, Vec<Case>>) {
    for(key, value) in processed_events.into_iter() {
        print!("{}: ", key.name);
        println!("{}", value.iter().format(", "));
    }
}

fn get_cli_file_name() -> Result<String, &'static str> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Exiting program, incorrect number of command line arguments (2) specified.");
        process::exit(1);
    }
    if !Path::new(&args[1]).exists() {
        println!("Second CLI argument {} does not exist as a file", args[1]);
        process::exit(1);
    }
    println!("Opening events file {} now\n", args[1].to_string());
    return Ok(args[1].to_string());
}

fn main() {
    println!("Welcome to the Pathology Parser!\n");
    let file_name = get_cli_file_name().unwrap();
    let file = File::open(file_name).expect("Unable to open");
    let reader = BufReader::new(file);
    let obj:Vec<Event> = serde_json::from_reader(reader).unwrap();
    let processed_data: HashMap<Patient, Vec<Case>>  = process_events(obj).unwrap();
    print_events(processed_data);
    
    println!("\nThank you for using the pathology parser!");
}
