use std::env;
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn main() {
    let args = parse_args();
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 5 {
        print_usage(&args[0]);
        eprintln!("{} wrong number of arguments: expected 4 but got {}.",
                  "Error".red().bold(), args.len()-1);
        std::process::exit(1);
    }

    Arguments {
        target: args[1].clone(),
        replacement: args[2].clone(),
        filename: args[3].clone(),
        output: args[4].clone(),
    }
}

fn print_usage(app_name: &str) {
    eprintln!("{} - change ocurrences of one string into another",
              app_name.green());
    eprintln!("Usage: {} <target> <replacement> <INPUT> <OUTPUT>",
              app_name);
}
