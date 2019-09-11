// 这次实现一个交叉链表

//list1 -> A ---+
//              |
//              v
//list2 ------> B -> C -> D
//              ^
//              |
//list3 -> X ---+
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    // append to front
    pub fn append(&self, elem: T) -> List<T> {
        let new_head = Rc::new(Node {
            elem: elem,
            next: self.head.clone(),
        });
        List {
            head: Some(new_head),
        }
    }

    // return new List which removed head
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|e| {
                e.next.clone()
            })
        }
        // or  List{
        //            head:self.head.as_ref().map_or(None, |e|{
        //                e.next.clone()
        //            })
        //        }
    }

    // peek the head
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|e| &e.elem)
    }
}

pub struct Iter<'a, T> {
    nextv: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            nextv: self.head.as_ref().map(|e| &(**e)),
        }
    }
}

impl<'a,T> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        self.nextv.take().map(|e|{
            self.nextv = e.next.as_ref().map(|e| &(**e));
            & e.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self){
        let _ = self.head.take();
    }
}

#[test]
fn basics1() {
    let list = List::new();
    assert_eq!(list.head(), None);

    let list = list.append(1).append(2).append(3);
    assert_eq!(list.head(), Some(&3));

    let list = list.tail();
    assert_eq!(list.head(), Some(&2));

    let list = list.tail();
    assert_eq!(list.head(), Some(&1));

    let list = list.tail();
    assert_eq!(list.head(), None);

    // Make sure empty tail works
    let list = list.tail();
    assert_eq!(list.head(), None);
}

#[test]
fn iter1() {
    let list = List::new().append(1).append(2).append(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

#[test]
fn two_iter1(){
    let list = List::new().append(1).append(2).append(3);

    let mut iter1 = list.iter();
    let mut iter2 = list.iter();
    assert_eq!(iter1.next(), Some(&3));
    assert_eq!(iter2.next(), Some(&3));
    assert_eq!(iter1.next(), Some(&2));
    assert_eq!(iter2.next(), Some(&2));
    assert_eq!(iter1.next(), Some(&1));
    assert_eq!(iter2.next(), Some(&1));
}

fn main() {
    println!("Hello, world!");
}
