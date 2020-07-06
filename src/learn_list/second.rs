use std::mem;

pub struct SecondList {
    head: Link,
}

struct SecondNode {
    data: i32,
    next: Link,
}

type Link = Option<Box<SecondNode>>;

impl SecondList {
    pub fn new() -> Self {
        SecondList { head: Link::None }
    }

    pub fn push(&mut self, val: i32) {
        //TODO
        let node = Box::new(SecondNode {
            data: val,
            next: self.head.take(),
        });
        self.head = Link::Some(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
}

impl Drop for SecondList {

fn drop(&mut self) { 
    // todo make an iterative drop 

    // first do the replace => grab ownership of th
    //Get the box iteratively and bring it out of scope
    let mut curr_node = self.head.take();  //mem::replace(&mut self.head, Link::None);
    // while loop 
    while let Link::Some(mut boxed_node) = curr_node {
        // grab the next and set it equal to 'Empty' so no furhter drops are made
        curr_node =boxed_node.next.take(); // mem::replace(&mut boxed_node.next,  Link::None);
    }
 }
}


mod test { 
    use super::SecondList;
    
    #[test]
    pub fn empty_list_check(){
        // TODO Make test babyeee
        let mut first_list = SecondList::new();
        assert_eq!(first_list.pop(), None);
    }

    #[test]
    pub fn push_pop_test() {
        let mut first_list = SecondList::new();

        first_list.push(1);
        first_list.push(2); 
        first_list.push(3);

        assert_eq!(first_list.pop(), Some(3));
        assert_eq!(first_list.pop(), Some(2));
        assert_eq!(first_list.pop(), Some(1));
    }
}
