mod build;
mod new;

use build::build;
use new::new;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("please provide an actual argument!");
        exit(0);
    }

    if args[1] == "build" {
        build();
    } else if args[1] == "new" {
        new(args).unwrap();
    } else {
        println!("this is not a valid argument, do --help for all arguments");
    }
}
