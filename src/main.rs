use std::env;

fn main() {
    println!("meow");

    let args: Vec<String> = env::args().collect();
    let arg = &args[0];

    if arg == "build" {
        println!("meow");
    }
}
