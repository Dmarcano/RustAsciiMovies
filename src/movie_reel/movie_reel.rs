pub trait MovieReel { 
    pub fn push_frame_to_tail(&mut self);

    pub fn push_frame_to_head(&mut self);

    pub fn pop_frame_from_tail(&mut self) -> Option<(i32,String)> ;

    pub fn pop_frame_from_head(&mut self) -> Option<(i32,String)> ;

    pub fn get_size(&mut self) -> usize;
}