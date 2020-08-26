mod movie_reel;
mod learn_list;

use learn_list::first_list;
use crate::movie_reel::play_movie::run;



fn main() {
    let file_name = String::from("test");
    run(file_name)
}
