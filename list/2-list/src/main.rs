// https://rust-unofficial.github.io/too-many-lists/second-peek.html
#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, v: T) {
        let new_node = Box::new(Node {
            elem: v,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|v| {
            let value = v.elem;
            self.head = v.next;
            value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|v| &v.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|v| &mut v.elem)
    }
}

// 我实现的比这个代码更少 https://rust-unofficial.github.io/too-many-lists/second-iter.html
// 不推荐把 & Option<...> &mut Option<...> 这种定义
/*
pub struct Iter<'a, T> {
    next: &'a Link<T>,
}
impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: &self.head }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.as_ref().map(|v| {
            // v = &std::boxed::Box<Node<T>>
            self.next = &v.next;
            &v.elem
        })
    }
}
*/

pub struct Iter<'a, T> {
    nextv: Option<&'a Node<T>>,
}
impl <T> List<T> {
    pub fn iter(&self) -> Iter<'_,T>{
        Iter{
            nextv: self.head.as_ref().map(|e|& **e)
        }
    }
}
impl<'a,T> Iterator for Iter<'a,T> {
    type Item = &'a T;
    fn next(&mut self)->Option<Self::Item> {
        self.nextv.take().map(|e|{
            self.nextv = e.next.as_ref().map(|e0|{& **e0});
            & e.elem
        })
    }
}

// the way in https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html
pub struct IterMut<'a,T> {
    nextv:Option<&'a mut Node<T>>
}
impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_,T> {
        IterMut{
            nextv:self.head.as_mut().map(|e|{
                // node = &mut std::boxed::Box<Node<T>>
                &mut **e
            })
        }
    }
}
impl<'a,T> Iterator for IterMut<'a,T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.nextv.take().map(|e|{
            self.nextv = e.next.as_mut().map(|e0|{
                &mut **e0
            });
            &mut e.elem
        })
    }
}


pub struct IntoIter<T> {
    nextv: List<T>,
}
impl <T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter{
            nextv:self,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.nextv.pop()
    }
}



#[test]
fn test_list1() {
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

#[test]
fn test_list_string() {
    let mut list = List::new();

    assert_eq!(list.pop(), None);

    list.push(String::from("a"));
    list.push(String::from("b"));
    list.push(String::from("c"));

    assert_eq!(list.pop(), Some(String::from("c")));
    assert_eq!(list.pop(), Some(String::from("b")));

    list.push(String::from("e"));
    list.push(String::from("f"));

    assert_eq!(list.pop(), Some(String::from("f")));
    assert_eq!(list.pop(), Some(String::from("e")));

    assert_eq!(list.pop(), Some(String::from("a")));
    assert_eq!(list.pop(), None);
}

#[test]
fn test_peek_string() {
    let mut list = List::new();

    list.push(String::from("a"));
    assert_eq!(list.peek(), Some(&String::from("a")));

    assert_eq!(list.pop(), Some(String::from("a")));
    assert_eq!(list.pop(), None);
}

#[test]
fn test_peek1() {
    let mut list = List::new();
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    list.push(1);
    list.push(2);
    list.push(3);

    assert_eq!(list.peek(), Some(&3));
    assert_eq!(list.peek_mut(), Some(&mut 3));

    list.peek_mut().map(|value| *value = 42);
    assert_eq!(list.peek(), Some(&42));
    assert_eq!(list.peek_mut(), Some(&mut 42));
}

#[test]
fn test_list2() {
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

#[test]
fn test_iter_i32() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));

    // we also can inmut-borrow here
    println!("list={:?}", list);

    assert_eq!(list.pop(), Some(3));
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
    assert_eq!(list.pop(), None);
}

#[test]
fn test_iter_string() {
    let mut list = List::new();
    list.push(String::from("a"));
    list.push(String::from("b"));
    list.push(String::from("c"));

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&String::from("c")));
    assert_eq!(iter.next(), Some(&String::from("b")));
    assert_eq!(iter.next(), Some(&String::from("a")));
    assert_eq!(iter.next(), None);

    assert_eq!(list.pop(), Some(String::from("c")));
    assert_eq!(list.pop(), Some(String::from("b")));
    assert_eq!(list.pop(), Some(String::from("a")));
    assert_eq!(list.pop(), None);
}

#[test]
fn test_two_iter() {
    let mut list = List::new();
    list.push(String::from("a"));
    list.push(String::from("b"));
    list.push(String::from("c"));

    let mut iter1 = list.iter();
    let mut iter2 = list.iter();
    assert_eq!(iter1.next(), Some(&String::from("c")));
    assert_eq!(iter2.next(), Some(&String::from("c")));
    assert_eq!(iter1.next(), Some(&String::from("b")));
    assert_eq!(iter2.next(), Some(&String::from("b")));
    assert_eq!(iter1.next(), Some(&String::from("a")));
    assert_eq!(iter2.next(), Some(&String::from("a")));
    assert_eq!(iter1.next(), None);

    assert_eq!(list.pop(), Some(String::from("c")));
    assert_eq!(list.pop(), Some(String::from("b")));
    assert_eq!(list.pop(), Some(String::from("a")));
    assert_eq!(list.pop(), None);
}

#[test]
fn test_iter_mut1() {
    let mut list = List::new();
    list.push(String::from("a"));
    list.push(String::from("b"));
    list.push(String::from("c"));

    let _ = list
        .iter_mut()
        .map(|x| x.push_str(&String::from("1")))
        .collect::<Vec<()>>();

    assert_eq!(list.pop().unwrap(), String::from("c1"));
}

#[test]
fn test_into_iter1(){
    let mut list = List::new();
    list.push(1); list.push(2); list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

fn main() {
    let mut list = List::new();
    list.push(String::from("a"));
    println!("list={:?}", list);
}
