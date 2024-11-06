mod build;
mod new;

use build::build;
use new::new;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args[1] == "build" {
        build();
    } else if args[1] == "new" {
        let olifantje = new(args);
    } else {
        println!("this is not a valid argument, do --help for all arguments");
    }
}
