use std::mem;

pub struct FirstLinkedList {
    head: Link,
}

struct FirstLLNode {
    data: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<FirstLLNode>),
}

impl FirstLinkedList {
    pub fn new() -> Self {
        FirstLinkedList { head: Link::Empty }
    }

    pub fn push(&mut self, val: i32) {
        //TODO
        let node = Box::new(FirstLLNode {
            data: val,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }
}

impl Drop for FirstLinkedList {

fn drop(&mut self) { 
    // todo make an iterative drop 

    // first do the replace => grab ownership of th
    //Get the box iteratively and bring it out of scope
    let mut curr_node =  mem::replace(&mut self.head, Link::Empty);
    // while loop 
    while let Link::More(mut boxed_node) = curr_node {
        // grab the next and set it equal to 'Empty' so no furhter drops are made
        curr_node = mem::replace(&mut boxed_node.next,  Link::Empty);
    }
 }
}


mod test { 
    use super::FirstLinkedList;
    
    #[test]
    pub fn empty_list_check(){
        // TODO Make test babyeee
        let mut first_list = FirstLinkedList::new();
        assert_eq!(first_list.pop(), None);
    }

    #[test]
    pub fn push_pop_test() {
        let mut first_list = FirstLinkedList::new();

        first_list.push(1);
        first_list.push(2); 
        first_list.push(3);

        assert_eq!(first_list.pop(), Some(3));
        assert_eq!(first_list.pop(), Some(2));
        assert_eq!(first_list.pop(), Some(1));
    }
}
