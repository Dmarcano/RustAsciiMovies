mod learnList;

use learnList::firstList;

fn loadMovie() {
    /*
    This function takes in a filename path and either returns an Error that the file was not found or the
    reference to a List which has the loaded movie file.
    */
}

fn playMovie() {
    /*
    This function takes ownership of a List struct representing a movie and plays the movie
    */
}

fn main() {
    let first_ll = firstList::FirstLinkedList::new();
    println!("Hello, world!");
}
