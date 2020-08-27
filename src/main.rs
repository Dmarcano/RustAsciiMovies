mod movie_reel;
mod learn_list;

use crate::movie_reel::play_movie::run;
use std::path::Path;
//use clap::{Arg, App, SubCommand};


fn main() {
    let path =Path::new("public/file2.txt");
    run(path)
}
