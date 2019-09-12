
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

#[derive(Debug)]
struct NodeMutRef<'a> {
    n: Option<&'a mut Node>,
}

impl <'a> NodeMutRef<'a> {

    fn new(l: &'a mut Node) -> Self{
        Self{
            n:Some(l),
        }
    }

    // 步骤拆分 // 测试是否能够做到链表后移  // 竟然可以
    fn test_move_next(&mut self) -> &mut Self {
        self.n.take().map(|e|{
            self.n = e.n.as_mut().map(|e0| {
                // e0 = `&mut std::boxed::Box<Link>`
                &mut (**e0)
            });
        });
        self
    }

    // 步骤拆分 // 测试是否能够做到返回可修改引用 // 可以
    fn test_mut(&mut self) -> Option<&mut String> {
        self.n.as_mut().map(|e|{
            //self.n = Some(e);
            &mut e.s
        })
    }
}


#[test]
fn test1(){
    let mut head = Node::new(String::from("aaa"));
    let tail = Box::new(Node::new(String::from("bbb")));
    head.n = Some(tail);

    let mut iter = NodeMutRef::new(&mut head);
    let mut s0 = iter.test_mut();
    s0.map(|e| *e = format!("--{}",e));
    assert_eq!(iter.test_mut(), Some(&mut String::from("--aaa")));

    assert_eq!(iter.test_mut(), Some(&mut String::from("--aaa")));


    iter.test_move_next();
    assert_eq!(iter.test_mut(), Some(&mut String::from("bbb")));

    iter.test_move_next();
    assert_eq!(iter.test_mut(), None);
}

fn main(){


}
