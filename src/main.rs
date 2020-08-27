mod movie_reel;

use crate::movie_reel::play_movie::run;
use std::path::Path;
use clap::{App, load_yaml};


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let path = Path::new(matches.value_of("movie").unwrap());
    run(path)
}
