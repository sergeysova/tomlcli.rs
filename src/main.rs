#[macro_use]
extern crate clap;
extern crate colored;
extern crate toml;

use std::fs::File;
use std::io::prelude::Read;
use toml::Value;
use colored::*;

/// Immediatly recursive print value
fn print_toml(value: &Value, parent_name: Option<&String>) {
    match *value {
        Value::String(ref string) => {
            print!("{}{}{}", "\"".green(), string.green(), "\"".green());
        }
        Value::Float(ref float) => {
            print!("{}", float.to_string().purple());
        }
        Value::Integer(ref integer) => {
            print!("{}", integer.to_string().purple());
        }
        Value::Boolean(ref boolean) => {
            match *boolean {
                true => print!("{}", "true".yellow()),
                false => print!("{}", "false".yellow()),
            }
        }
        Value::Datetime(ref datetime) => {
            print!("{}", datetime.to_string().magenta());
        }
        Value::Array(ref array) => {
            print!("{}", "[".blue());
            for (index, val) in array.iter().enumerate() {
                print_toml(&val, None);

                if index < (array.len() - 1) {
                    print!("{} ", ",".blue());
                }
            }
            print!("{}", "]".blue());
        }
        Value::Table(ref table) => {
            for (name, value) in table
                    .iter()
                    .filter(|&(_, ref value)| value.type_str() != "table") {
                match *value {
                    Value::Table(..) => {}
                    _ => {
                        print!("{} = ", name.blue());
                        print_toml(value, None);
                        println!();
                    }
                }
            }
            for (name, value) in table
                    .iter()
                    .filter(|&(_, ref value)| value.type_str() == "table") {
                match *value {
                    Value::Table(ref table) => {
                        #[allow(unused_assignments)]

                        let mut fullname = if let Some(parent_name) = parent_name {
                            format!("{}.{}", parent_name, name)
                        } else {
                            name.clone()
                        };

                        if table
                               .iter()
                               .filter(|&(_, ref value)| value.type_str() != "table")
                               .count() > 0 {
                            println!();
                            println!("{}{}{}", "[".blue(), fullname.white(), "]".blue());
                        }
                        print_toml(value, Option::from(&fullname));
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Search path like 'foo.bar.baz' in Value::Table recursively
fn select_path<'a>(value: &'a Value, path: &String) -> Option<&'a Value> {
    let chunks = path.split('.');
    let mut current = Some(value);

    for chunk in chunks {
        if let Some(current_value) = current {
            match *current_value {
                Value::Table(ref table) => {
                    if let Some(found) = table.get(chunk) {
                        current = Some(found);
                    } else {
                        current = None;
                        break;
                    }
                }
                Value::Array(ref array) => {
                    if let Ok(index) = chunk.parse::<usize>() {
                        if index < array.len() {
                            let element = &array[index];
                            current = Some(element);
                        }
                        else {
                            current = None;
                            break;
                        }
                    } else {
                        current = None;
                        break;
                    }
                }
                _ => {
                    current = None;
                    break;
                }
            }
        } else {

        }
    }

    current
}

fn main() {
    let matches = clap_app!(myapp =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg PATH: +required "Path to TOML file relative to current working directory")
        (@arg QUERY: "Filter is a dot-separated path to a property or category")
        (@arg verbose: -V --verbose "Show debug info")
    )
            .get_matches();

    let verbose = matches.is_present("verbose");
    let path = matches.value_of("PATH").unwrap();

    match File::open(path) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content)
                .expect("Cannot read file");

            match content.parse::<Value>() {
                Ok(value) => {
                    if let Some(query) = matches.value_of("QUERY") {
                        if let Some(found) = select_path(&value, &query.to_string()) {
                            print_toml(found, None);
                            println!();
                        } else {
                            println!("{} Query '{}' not found",
                                     "Error:".red(),
                                     query.bold().yellow());
                            std::process::exit(1);
                        }
                    } else {
                        print_toml(&value, None);
                    }
                }
                Err(error) => {
                    println!("{} Cannot parse file '{}'",
                             "Error:".red(),
                             path.bold().yellow());

                    if verbose { println!("{}", error); }
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            println!("{} Cannot open file '{}'",
                     "Error:".red(),
                     path.bold().yellow());

            if verbose {
                println!("{}", error);
            }
            std::process::exit(1);
        }
    }
}
