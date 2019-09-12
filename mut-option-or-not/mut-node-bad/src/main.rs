#[derive(Debug)]
struct Node {
    s: String,
    n: Option<Box<Node>>,
}

impl Node {
    fn new(s :String) -> Self {
        Self{
            s:s,
            n:None
        }
    }
}

// I know a way in https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html
// for differ with IterMut, we named NodeMutRef
#[derive(Debug)]
struct NodeMutRef<'a> {
    n: &'a mut Option<Box<Node>>, //  I know Option<&'a mut Node> is ok, but why cannot this?
}

impl<'a> NodeMutRef<'a> {

    fn new(l :&'a mut Option<Box<Node>>) -> Self {
        Self{
            n:l,
        }
    }

    // 步骤拆分 // 仅仅看是否能够做到链表后移 // 做不到
    fn test_move_next(&mut self) {
        // `old_node` does not live long enough
        let mut old_node = self.n.take();
        old_node.as_mut().map(|e|{
            self.n = &mut e.n;
        });
    }

    // 步骤拆分 // 仅仅看是否能够返回可修改引用 // 是能够做到的
    fn test_mut(&mut self) -> Option<&mut String> {
        let r = self.n.as_mut().map(|e| {
            &mut (**e).s
        });
        return r;
    }
}


#[test]
fn test1(){
    let mut head = Node::new(String::from("aaa"));
    let tail = Box::new(Node::new(String::from("bbb")));
    head.n = Some(tail);

    let mut iter = NodeMutRef::new(&mut head.n);
    let mut s0 = iter.test_mut();
    s0.map(|e| *e = format!("--{}",e));
    assert_eq!(iter.test_mut(), Some(&mut String::from("--bbb")));

    assert_eq!(iter.test_mut(), Some(&mut String::from("--bbb")));

    iter.test_move_next();
    assert_eq!(iter.test_mut(), None);
}

fn main() {
}
