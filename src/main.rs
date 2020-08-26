mod movie_reel;
mod learn_list;

use learn_list::first_list;
use crate::movie_reel::play_movie::run;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use clap::{Arg, App, SubCommand};


fn main() {
    let path =Path::new("public/file2.txt");
    run(path)
}
