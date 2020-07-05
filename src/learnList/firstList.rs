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
