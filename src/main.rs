use std::{env};
use url_pocket_in_rust::{save_url, show_url};
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = configuration(&args);

    if args[1] == "save" {
        match save_url(config.url, config.url_type, config.desc) {
            Ok(_) => println!("Url Saved Successfully ✅"),
            Err(e) => eprintln!("Url was not saved {}", e),
        }
    } else if args[1] == "show" {
        match show_url(config.url_type) {
            Ok(_) => println!("All Url from {} showed✅", args[2]),
            Err(e) => eprintln!("It's not possible to show the urls {}", e),
        }
    } else if args[1] == "help" {
        const SRC_URL: &str = "https://github.com/Muriyoku/url-pocket-in-rust";
        
        let cli_commands: [&'static str; 2] = [
            "cargo run show -> show all urls from a pocket",
            "cargo run save -> save a url on a pocket\n"
        ];

        println!("All availables commands:\n");
        
        for cli_command in cli_commands {
            println!("{}", cli_command);
        }

        println!("more information in: {}", SRC_URL.bright_blue())
    }
}

struct Config {
    url_type: String, 
    desc: String, 
    url: String, 
}

impl Config {
    fn new() -> Config {
        return Config { 
            url_type: String::new(), 
            desc: String::from("No Description added"), 
            url: String::new() 
        };
    }
}

fn configuration(configs: &Vec<String>) -> Config {
    let mut configuration = Config::new();

    if configs[1] == "show" {
        configuration.url_type = configs[2].clone().to_string();
    } 
     
    if configs[1] == "save" {
        configuration.url_type = configs.get(2).cloned().unwrap_or_default();
        configuration.url = configs.get(3).cloned().unwrap_or_default();
        configuration.desc = configs.get(4).cloned().unwrap_or_default();
    }

    return configuration;
}
