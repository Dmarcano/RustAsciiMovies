pub trait MovieReel { 
    fn push_frame_to_tail(&mut self);

    fn push_frame_to_head(&mut self);

    fn pop_frame_from_tail(&mut self) -> Option<(u64,String)> ;

    fn pop_frame_from_head(&mut self) -> Option<(u64,String)> ;

     fn get_size(&mut self) -> usize;
}