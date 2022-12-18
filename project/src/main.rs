//https://github.com/nightraiser/daily-code-rust/blob/master/json-reading/src/main.rs
//run cargo check or cargo run or ./project from target/debug project directory
//https://tokio.rs/tokio/tutorial/spawning

//delete line ctrl and X.

#![allow(unused)]

use std::path::Path;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]

struct Track{
    title: String,
    link: String,
    
}

fn main() {

    println!("Starting Program");


    let json_file_path="import.json";
    let file = File::open(json_file_path).expect("file not found");
    let tracks:Vec<Track> = serde_json::from_reader(file).expect("error while reading");

    for entry in tracks {
        println!("Attempting to generate mp3 for {}", entry.title);
    }
    println!("Program Completed");
}
