extern crate colored;
extern crate toml;

use toml::Value;
use colored::*;

fn print_toml(value: &Value, has_parent: bool, parent_name: &String) {
    match *value {
        Value::String(ref string) => {
            print!("{}{}{}", "\"".green(), string.green(), "\"".green());
        },
        Value::Float(ref float) => {
            print!("{}", float.to_string().purple());
        },
        Value::Integer(ref integer) => {
            print!("{}", integer.to_string().purple());
        }
        Value::Boolean(ref boolean) => {
            match *boolean {
                true => print!("{}", "true".yellow()),
                false => print!("{}", "false".yellow()),
            }
        },
        Value::Datetime(ref datetime) => {
            print!("{}", datetime.to_string().magenta());
        },
        Value::Array(ref array) => {
            print!("{}", "[".blue());
            for (index, val) in array.iter().enumerate() {
                print_toml(&val, false, &String::new());

                if index < (array.len() - 1) {
                    print!("{} ", ",".blue());
                }
            }
            print!("{}", "]".blue());
        },
        Value::Table(ref table) => {
            for (name, value) in table.iter().filter(|&(_, ref value)| value.type_str() != "table") {
                match *value {
                    Value::Table(..) => {},
                    _ => {
                        print!("{} = ", name.blue());
                        print_toml(value, false, &String::new());
                        println!("");
                    }
                }
            }
            for (name, value) in table.iter().filter(|&(_, ref value)| value.type_str() == "table") {
                match *value {
                    Value::Table(ref table) => {
                        #[allow(unused_assignments)]
                        let mut fullname = String::new();

                        if has_parent {
                            fullname = format!("{}.{}", parent_name, name);
                        }
                        else {
                            fullname = name.clone();
                        }
                        if table.iter().filter(|&(_, ref value)| value.type_str() != "table").count() > 0 {
                            println!("");
                            println!("{}{}{}", "[".blue(), fullname.white(), "]".blue());
                        }
                        print_toml(value, true, &fullname);
                    },
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let raw = r#"
root = 1123
        [package]
name = "tomlcli"
version = "0.1.0"
authors = ["Sergey Sova <mail@sergeysova.com>"]
license = "MIT"
repository = "https://github.com/sergeysova/tomlcli.rs"
documentation = "https://docs.rs/tomlcli/"
readme = "README.md"
description = "Parse and query toml files"
keywords = ["toml", "cli", "parser", "print", "console", "terminal"]

[dependencies]
toml = "0.4.5"
colored = "1.6.0"
demo = 1994-02-12
ini = 1
foo = false
bar = true
nfl = 2.51
[dependencies.naf]
version = 123
[foo.bar.baz]
baf = [1.2, 3.4, 5.6]

    "#;

    let value: Value = raw.parse().unwrap();
    print_toml(&value, false, &String::new());
}
