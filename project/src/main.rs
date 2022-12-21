//https://github.com/nightraiser/daily-code-rust/blob/master/json-reading/src/main.rs
//run cargo check or cargo run or ./project from target/debug project directory
//https://tokio.rs/tokio/tutorial/spawning

//delete line ctrl and X.

#![allow(unused)]

use std::path::Path;
use std::fs::File;
use serde::Deserialize;
use std::process::Command;

//arg("%(title)s.%(ext)s")

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
      
        println!("link is {}",entry.link);
        let t = &entry.title;
             
        let status = Command::new("/bin/youtube-dl").arg("-x").arg("--audio-format").arg("mp3").arg("-o").arg("%(".to_owned() + "title" + ")s.%(ext)s").arg("--restrict-filenames")
                            .arg(entry.link)
                            .status()
                            .expect("failed to execute process");

        println!("process finished with: {status}");

        assert!(status.success());
        

    }


    println!("Program Completed");
}
