// 疑问 来自 https://www.zhihu.com/question/325569047/answer/692216122

#[derive(Debug)]
struct A {
    //name: String, // not impl Copy trait
    age: i32,
}

impl A {
    // 这里是 move 吗 ？
    fn get_owned(self) -> Self {
        println!("get owned called");
        self
    }

    fn get_mut(&mut self) -> &mut Self {
        self
    }
}

#[cfg(windows)]
fn test1() {
    let mut a = A {
        // name: String::from("hello"),
        age: 3,
    };

    println!("a={:?}", a);
    // 这里认为是一个可变借用
    let b = a.get_mut();
    println!("b={:?}", b);
    let c = b.get_owned();
    // 上面有可变借用，这里就不能有不可变借用
    //    println!("a={:?}", a);
    println!("b={:?}", b);
    println!("c={:?}", c);

    println!("change b");
    b.age = 4;
    println!("b={:?}", b);
    println!("c={:?}", c);
}

#[cfg(ignore)]
fn test_cannot_borrow() {
    let mut a = A {
        // name: String::from("hello"),
        age: 3,
    };
    println!("a={:?}", a);
    {
        //   value moved here
        let b = a.get_owned();
        // cannot borrow
        //println!("a={:?}", a);
        println!("b={:?}", b);
    }
    // cannot borrow
    println!("a={:?}", a);
}

fn main() {
    test_cannot_borrow();
}
