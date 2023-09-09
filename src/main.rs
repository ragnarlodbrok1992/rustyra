use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    println!("Rustyra compiler - development version.");

    // Checking command line arguments
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // Check current os path
    let _path = env::current_dir()?;
    let mut is_source_provided: bool = false;
    let mut source_file: File;
    let mut source_file_path = String::new();
    // println!("The current directory is {}", path.display());

    // Get command line argument and check if it is rustyra source code (file that exist
    // and ends in .rustyra type)
    for cmd in args {
        println!("cmd: {}", cmd);
        // Check if file has type rustyra (extension)
        if cmd.ends_with(".rustyra") {
            is_source_provided = true;
            source_file_path = cmd;
            break;
        }
    }

    if is_source_provided {
        println!("Source code provided!");
        // Do something with source code
        // 1. Open file and load it's contents
        source_file = File::open(source_file_path)?;
        let mut source_file_contents = String::new();
        source_file.read_to_string(&mut source_file_contents)?;
        // return Ok(());
    } else {
        // No source file is provided - quit
    }

    println!("Everything went good - exiting.");
    return Ok(());
}

