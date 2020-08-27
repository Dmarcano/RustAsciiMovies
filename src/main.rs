mod movie_reel;

use crate::movie_reel::play_movie::run;
use std::path::Path;
//use clap::{Arg, App, SubCommand};


fn main() {
    let path =Path::new("public/sw1.txt");
    run(path)
}
