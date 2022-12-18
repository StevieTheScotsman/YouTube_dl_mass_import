//https://github.com/nightraiser/daily-code-rust/blob/master/json-reading/src/main.rs

#![allow(unused)]

use std::path::Path;
use std::fs::File;
use serde::Deserialize;



fn main() {

    let json_file_path="import.json";
    let file = File::open(json_file_path);
}
