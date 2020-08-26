mod movie_reel;
mod learn_list;

use learn_list::first_list;
use crate::movie_reel::play_movie::run;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


fn main() {
    let path =Path::new("public/file2.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    // run(file_name)

}
