extern crate clap;
extern crate serde;
extern crate serde_json;
use clap::{Arg, App};
use std::fs::File;
use std::fs::canonicalize;
use std::io::Read;
use std::io::Write;
use std::process::exit;
use std::env;
use serde_json::Value;
use serde_json::value::to_value;

fn main() {
    let matches = App::new("memo")
        .version("1.0")
        .author("Shashank Kambhampati <shashankk@utexas.edu>")
        .about("Stores strings for easy access")
        .arg(Arg::with_name("set")
             .short("s")
             .long("set")
             .requires_all(&["key", "value"])
             .help("Create a new mapping or overwrite an old one"))
        .arg(Arg::with_name("file")
             .short("f")
             .long("file")
             .requires("set")
             .help("Expand passed in value to full file or directory"))
        .arg(Arg::with_name("delete")
             .short("d")
             .long("delete")
             .requires("key")
             .conflicts_with("set")
             .help("Delete an existing mapping"))
        .arg(Arg::with_name("list")
             .short("l")
             .long("list")
             .conflicts_with_all(&["set", "delete"])
             .help("List existing mappings"))
        .arg(Arg::with_name("key")
             .value_name("KEY")
             .help("Key of the mapping to use")
             .required_unless("list"))
        .arg(Arg::with_name("value")
             .value_name("VALUE")
             .help("Used for set - value to set the key to"))
        .get_matches();

    let mut config = read_config();
    let mut modified = false;

    if matches.is_present("list") {
        for val in config.as_object().expect("Config file is invalid") {
            println!("{}: {}", val.0, val.1);
        }
    } else if matches.is_present("set") {
        let key = matches.value_of("key").unwrap();
        let mut value: String = matches.value_of("value").unwrap().into();

        if matches.is_present("file") {
            let path = canonicalize(value).expect("Provided value is not valid path");
            value = path.into_os_string().into_string().expect("Path contains invalid characters");
        }
        
        config.as_object_mut().expect("Config file is invalid").insert(key.into(), to_value(value).unwrap());
        modified = true;
    } else if matches.is_present("delete") {
        let key = matches.value_of("key").unwrap();
        config.as_object_mut().expect("Config file is invalid").remove(key);
        modified = true;
    } else {
        let key = matches.value_of("key").unwrap();

        if let Some(value) = config[key].as_str() {
            println!("{}", value);
        } else {
            println!("Key {} not set", key);
            exit(1);
        }
    }

    if modified {
        write_config(config);
    }
}

fn read_config() -> Value {
    let config_file_path = env::home_dir().expect("Home directory not set").join(".memorc");
    let config_file = File::open(config_file_path);

    let mut buffer = String::new();
    if let Ok(mut file) = config_file {
        file.read_to_string(&mut buffer).expect("Could not open ~/.memorc");
    } else {
        buffer = "{}".into();
    }

    serde_json::from_str(&buffer).expect("~/.memorc not in JSON format")
}

fn write_config(contents: Value) {
    let config_file_path = env::home_dir().expect("Home directory not set").join(".memorc");
    let mut config_file = File::create(config_file_path).expect("Did not have write permissions to config file");

    config_file.write(contents.to_string().as_bytes()).expect("Did not have write permissions to config file");
}

