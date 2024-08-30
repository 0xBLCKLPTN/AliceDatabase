pub mod atypes;
pub mod configurator;
use atypes::*;
use configurator::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    //let file_path = &args[1];
    let file_path = "/home/twofacedjanus/Documents/AliceDatabase/config_template.json";
    let config = configurator::read_config(file_path);
    println!("Hello, world!");
}
