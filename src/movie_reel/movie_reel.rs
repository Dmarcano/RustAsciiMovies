pub trait MovieReel<T> { 
    fn pop_frame_from_tail(&mut self) -> Option<T> ;

    fn pop_frame_from_head(&mut self) -> Option<T> ;
}