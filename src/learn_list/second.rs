

pub struct SecondList<T> {
    head: Link<T>,
}

struct SecondNode<T> {
    data: T,
    next: Link<T>,
}

type Link<T> = Option<Box<SecondNode<T>>>;

impl<T> SecondList<T> {
    pub fn pop_tail(&mut self) -> Option<T> {

        let sample = vec![1,2];
        unimplemented!();
    }

    pub fn new() -> Self {
        SecondList { head: None }
    }

    pub fn push(&mut self, val: T) {
        //TODO
        let node = Box::new(SecondNode {
            data: val,
            next: self.head.take(),
        });
        self.head = Link::Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(& self ) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.data
        } )
    }

    pub fn peek_as_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node|{
            &mut node.data
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter<'a>(& 'a self) -> Iter< 'a, T> {
        Iter{next : self.head.as_ref(). map(|node| &**node)}
    }
}

impl<T> Drop for SecondList<T> {

fn drop(&mut self) { 

    // first do the replace => grab ownership of th
    //Get the box iteratively and bring it out of scope
    let mut curr_node = self.head.take();  //mem::replace(&mut self.head, Link::None);
    // while loop 
    while let Link::Some(mut boxed_node) = curr_node {
        // takes the next node and set it equal to 'Empty' so no furhter drops are made
        curr_node =boxed_node.next.take(); // mem::replace(&mut boxed_node.next,  Link::None);
    }
 }
}

/**
Implementing Into Iter to iterate through the list
*/
pub struct IntoIter<T>(SecondList<T>); // this is a struct with a list as its only parameter

impl <T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/**
Implementing Iter. This is the start to using lifetimes
*/

// we start with a struct called Iter. This struct has a 'next' field which has
// an Option for a Reference of a SecondList of type T with lifetime 'a. Meaning that as long as
// the SecondList with 'a is around then the Iter must also be kept alive by rust compiler
pub struct Iter<'a, T> {
    next: Option<&'a SecondNode<T>>,
}

// We are now implementing the Iterator trait for the Iter
impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map( |node| {
            self.next = self.next.as_ref().map(|node | &**node);
            &node.data
        } )
    }
}
/**
Implementing Iter mut
*/

mod test { 
    use super::SecondList;
    
    #[test]
    pub fn empty_list_check(){
        // TODO Make test babyeee
        let mut first_list : SecondList<String> = SecondList::new();
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

    #[test]
    pub fn peek_test() {
        let mut list = SecondList::new();

        list.push(5);
        let peek_val = list.peek();
        assert_eq!(Some(&5), peek_val)
    }

    #[test]
    pub fn mut_peek_test() {
        let mut list = SecondList::new();
        list.push(2);
        list.peek_as_mut().map(|value| {
            *value = 42;
        });
        assert_eq!(list.peek_as_mut(), Some(&mut 42));
    }

    #[test]
    pub fn into_iter_test() {
        let mut list = SecondList::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next() , Some(3));
        assert_eq!(iter.next() , Some(2));
        assert_eq!(iter.next() , Some(1));
    }

    #[test]
    pub fn iter_test() {
        let mut list = SecondList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

}
