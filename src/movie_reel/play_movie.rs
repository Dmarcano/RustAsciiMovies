use super::movie_reel;
use std::io::stdout;
use std::{thread, time};
use std::io::Write;
use crate::movie_reel::movie_reel::MovieReel;
use std::fs::File;
use std::path::Path;

impl<T> MovieReel for Vec<T> {
    fn push_frame_to_tail(&mut self) {
        unimplemented!()
    }

    fn push_frame_to_head(&mut self) {
        unimplemented!()
    }

    fn pop_frame_from_tail(&mut self) -> Option<(u64, String)> {
        unimplemented!()
    }

    fn pop_frame_from_head(&mut self) -> Option<(u64, String)> {
        unimplemented!()
    }

    fn get_size(&mut self) -> usize {
        unimplemented!()
    }
}

pub fn load_movie(path : &Path) -> Vec<(u64, String)> {
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    unimplemented!("Not implemented creating a movie!")
}

pub fn play_movie(mut movie : impl movie_reel::MovieReel) {
    loop{
        match movie.pop_frame_from_head() {
            None => return,
            Some(out_count) => {
                let count : u64= out_count.0;
                let frame : String= out_count.1;
                 match stdout().flush()  {
                    Ok(_) => {}
                    Err(why) => panic!("couldn't flush command line! {}", why),
                 }
                print!("{}" , frame);
                let millis = time::Duration::from_millis(1000000*count/15);
                thread::sleep(millis);
            }
        }
    }
}

pub fn run(filename: &Path) {
    let movie_reel = load_movie(filename);
    play_movie(movie_reel);
}