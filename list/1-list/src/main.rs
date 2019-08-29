use std::mem;

#[derive(Debug)]
struct Node<T>
where
    T: Copy,
{
    elem: T,
    next: Link<T>,
}

#[derive(Debug)]
enum Link<T>
where
    T: Copy,
{
    Empty,
    More(Box<Node<T>>),
}

pub struct List<T>
where
    T: Copy,
{
    head: Link<T>,
}

impl<T> List<T>
where
    T: Copy,
{
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(new_node));
    }
    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T> Drop for List<T>
where
    T: Copy,
{
    fn drop(&mut self) {
        let mut link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = link {
            link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

fn test_list() {
    let mut list = List::new();

    // Check empty list behaves right
    assert_eq!(list.pop(), None);

    // Populate list
    list.push(1);
    list.push(2);
    list.push(3);

    // Check normal removal
    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.push(4);
    list.push(5);

    // Check normal removal
    assert_eq!(list.pop(), Some(5));
    assert_eq!(list.pop(), Some(4));

    // Check exhaustion
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

fn main() {
    test_list();
    println!("Hello, world!");
}
