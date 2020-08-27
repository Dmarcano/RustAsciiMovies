use std::io::stdout;
use std::{thread, time};
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::path::Path;
use std::collections::vec_deque::VecDeque;

use super::movie_reel;
use crate::movie_reel::movie_reel::MovieReel;


impl<T> MovieReel<T> for VecDeque<T> {
    fn pop_frame_from_tail(&mut self) -> Option<T> {
        self.pop_back()
    }

    fn pop_frame_from_head(&mut self) -> Option<T> {
        self.pop_front()
    }
}

pub fn load_movie(path : &Path) -> VecDeque<(u64, String)> {
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    // use the BufReader interface to read the file line by line
    let reader = BufReader::new(file); 

    let mut movie_vector : VecDeque<(u64, String)> = VecDeque::new();
    // if we can't read the file it is Ok to panic
    let mut count : u64 = 0; 
    let mut accum = String::new();
    for (idx, line) in reader.lines().map(|l| l.unwrap()).enumerate() { 
        match idx { 
            0 => {count = line.parse().unwrap()}
            x => {
                accum = accum + &line;
                if x%14 == 0{
                    movie_vector.push_back((count, accum.clone()));
                    accum = String::new();
                }
            } 
        } 
    }
    movie_vector
}

pub fn play_movie(mut movie : impl movie_reel::MovieReel<(u64, String)>) {
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

#[cfg(test)]
mod tests {
    #[test]
    fn movie_test() {
        

        assert_eq!(2 + 2, 4);
    }
}