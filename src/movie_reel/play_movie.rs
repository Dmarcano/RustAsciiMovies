use super::movie_reel;
use std::io::stdout;
use std::{thread, time};
use std::io::Write;

pub fn load_movie{filename : String} -> movie_reel::MovieReel {
    // TODO
}

pub fn play_movie(movie : movie_reel::MovieReel) {
    // TODO

    let size = movie.get_size();
    let mut (count, frame);

    for(i32 i =0; i < size; i++) {
        let result = movie.pop_frame_from_head();

        match result {
            None => return;  
            Some(out_count, out__frame) => (count, frame);
        }
        // if we get a  count and frame then play them
        stdout().flush();
        print!("{}" , frame);
        let millis = time::Duration::from_millis(1000000*count/15);
        thread::sleep(millis);
    }
}