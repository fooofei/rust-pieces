// 这个文件演示解构一个结构体中的成员 与 直接访问成员的用法区别

// ref/ ref mut 是用在 L-value 上的

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn destructuring_with_match() {
    let mut p = Person {
        name: String::from("bob"),
        age: 10,
    };

    println!("p= {:?}", p);

    let name0 = match p {
        Person {
            name: ref name1,
            age: _,
        } => name1,
    };

    println!("name0= {:?}", name0);
    println!("p={:?}", p);

    let name3 = match p {
        Person {
            name: ref mut name4,
            age: _,
        } => name4,
    };

    println!("name3={:?}", name3);
    *name3 = String::from("allen");

    println!("p={:?}", p);
}

fn destructuring_with_field() {
    let mut p = Person {
        name: String::from("bob"),
        age: 10,
    };

    println!("p= {:?}", p);

    // 1
    let name0 = &p.name;

    println!("name0= {:?}", name0);
    println!("p={:?}", p);

    // 2
    let name1 = &mut p.name;
    println!("name1= {:?}", name1);
    *name1 = String::from("allen");
    println!("p={:?}", p);

    // 3 与 1 等价
    let ref name3 = p.name;
    println!("name3 = {:?}", name3);
    println!("p={:?}", p);

    // 4 与 2 等价
    let ref mut name4 = p.name;
    *name4 = String::from("smith");
    println!("p= {:?}", p);
}

fn main() {
    println!("Hello, world!");
    destructuring_with_match();
    println!("destructuring_with_field -----");
    destructuring_with_field();
}
