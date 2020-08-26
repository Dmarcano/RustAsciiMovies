mod movie_reel;
mod learn_list;

use learn_list::first_list;
use crate::movie_reel::play_movie::run;



fn main() {
    let file_name = String::from("public/file2.txt");
    run(file_name)
}
