#[macro_use]
extern crate clap;
extern crate colored;
extern crate toml;

use std::fs::File;
use std::io::prelude::Read;
use toml::Value;
use colored::*;

fn magenta<T: Into<String>>(val: T, has_color: bool) -> impl std::fmt::Display {
    let value = val.into().magenta();

    if has_color { value } else { value.clear() }
}

fn green<T: Into<String>>(val: T, has_color: bool) -> impl std::fmt::Display {
    let value = val.into().green();

    if has_color { value } else { value.clear() }
}

fn purple<T: Into<String>>(val: T, has_color: bool) -> impl std::fmt::Display {
    let value = val.into().purple();

    if has_color { value } else { value.clear() }
}

fn yellow<T: Into<String>>(val: T, has_color: bool) -> impl std::fmt::Display {
    let value = val.into().yellow();

    if has_color { value } else { value.clear() }
}

fn blue<T: Into<String>>(val: T, has_color: bool) -> impl std::fmt::Display {
    let value = val.into().blue();

    if has_color { value } else { value.clear() }
}

fn white<T: Into<String>>(val: T, has_color: bool) -> impl std::fmt::Display {
    let value = val.into().white();

    if has_color { value } else { value.clear() }
}

fn red<T: Into<String>>(val: T, has_color: bool) -> impl std::fmt::Display {
    let value = val.into().red();

    if has_color { value } else { value.clear() }
}

/// Immediatly recursive print value
fn print_toml(value: &Value, parent_name: Option<&String>, has_color: bool) {
    match *value {
        Value::String(ref string) => {
            print!("{}{}{}",
                green("\"", has_color),
                green(string.to_string(), has_color),
                green("\"", has_color),
            );
        }
        Value::Float(ref float) => {
            print!("{}", purple(float.to_string(), has_color));
        }
        Value::Integer(ref integer) => {
            print!("{}", purple(integer.to_string(), has_color));
        }
        Value::Boolean(ref boolean) => {
            match *boolean {
                true => print!("{}", yellow("true", has_color)),
                false => print!("{}", yellow("false", has_color)),
            }
        }
        Value::Datetime(ref datetime) => {
            print!("{}", magenta(datetime.to_string(), has_color));
        }
        Value::Array(ref array) => {
            print!("{}", "[".blue());
            for (index, val) in array.iter().enumerate() {
                print_toml(&val, None, has_color);

                if index < (array.len() - 1) {
                    print!("{} ", blue(",", has_color));
                }
            }
            print!("{}", blue("]", has_color));
        }
        Value::Table(ref table) => {
            for (name, value) in table
                    .iter()
                    .filter(|&(_, ref value)| value.type_str() != "table") {
                match *value {
                    Value::Table(..) => {}
                    _ => {
                        print!("{} = ", blue(name.to_string(), has_color));
                        print_toml(value, None, has_color);
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
                            println!("{}{}{}", blue("[", has_color), white(fullname.to_string(), has_color), blue("]", has_color));
                        }
                        print_toml(value, Option::from(&fullname), has_color);
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
        (@arg no_color: -W --nocolor "Disable color output")
    )
            .get_matches();

    let verbose = matches.is_present("verbose");
    let has_color = !matches.is_present("no_color");
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
                            print_toml(found, None, has_color);
                            println!();
                        } else {
                            println!("{} Query '{}' not found",
                                     red("Error:", has_color),
                                     yellow(query, has_color));
                            std::process::exit(1);
                        }
                    } else {
                        print_toml(&value, None, has_color);
                    }
                }
                Err(error) => {
                    println!("{} Cannot parse file '{}'",
                             red("Error:", has_color),
                             yellow(path, has_color));

                    if verbose { println!("{}", error); }
                    std::process::exit(1);
                }
            }
        }
        Err(error) => {
            println!("{} Cannot open file '{}'",
                     red("Error:", has_color),
                     yellow(path, has_color));

            if verbose {
                println!("{}", error);
            }
            std::process::exit(1);
        }
    }
}
