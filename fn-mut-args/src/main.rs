// 这个文件用来演示函数参数是可变引用时的语法

// 作为结构体成员的时候
struct VecMutRef<'a> {
    mv: &'a mut Vec<String>,
}

// 在这个函数里怎么修改这个可变引用
// 在这个函数里怎么给另一个函数传递参数
fn mut_vec(v: &mut Vec<String>) {
    v.iter_mut().for_each(|v| {
        *v = format!("1{}", v);
    });
    // WANING: not &mut v
    mut_vec_again(v);
}

fn mut_vec_again(v: &mut Vec<String>) {
    v.iter_mut().for_each(|v| {
        *v = format!("2{}", v);
    });
}

fn mut_with_mut_ref(p: &mut VecMutRef) {
    mut_vec(p.mv);
}

pub fn mut_opt_box(v: &mut Option<Box<String>>) -> Option<&mut String> {
    v.as_mut().map(|v0| {
        // v0= `&mut std::boxed::Box<std::string::String>`
        &mut **v0
    })
}

// 作为结构体成员的时候 怎么修改
struct OptBoxMutRef<'a> {
    m: &'a mut Option<Box<String>>,
}

impl<'a> OptBoxMutRef<'a> {
    fn new(b: &'a mut Option<Box<String>>) -> Self {
        Self { m: b }
    }
    // 果然报错，这里 return 更改为 Option<&'a mut String>
    // 即便是不调用这里的代码，也是无法编译过去的
    // 报错依旧 Option<&'a mut String>
    fn mut_string(&mut self) -> Option<&mut String> {
        self.m.as_mut().map(|v0| &mut **v0)
    }
}

#[test]
fn test_mut_vec1() {
    let mut ss = vec![String::from("a"), String::from("b")];
    mut_vec(&mut ss);
    assert_eq!(ss[0], String::from("21a"));
    assert_eq!(ss[1], String::from("21b"));
}

#[test]
fn test_mut_vec2() {
    let mut ss = vec![String::from("a"), String::from("b")];
    let mut st = VecMutRef { mv: &mut ss };
    mut_with_mut_ref(&mut st);
    assert_eq!(ss[0], String::from("21a"));
    assert_eq!(ss[1], String::from("21b"));
}

#[test]
fn test_mut_opt_box1() {
    let mut v0 = Some(Box::new(String::from("aaa")));

    // or mut
    let v1 = mut_opt_box(&mut v0);
    v1.map(|e| {
        *e = format!("--{}", e);
    });

    assert_eq!(v0, Some(Box::new(String::from("--aaa"))));
}

#[test]
fn test_mut_opt_box2() {
    let mut v0 = Some(Box::new(String::from("aaa")));

    let mut v1 = OptBoxMutRef::new(&mut v0);
    v1.mut_string().map(|e| {
        *e = format!("--{}", e);
    });

    assert_eq!(v0, Some(Box::new(String::from("--aaa"))));
}

fn main() {}
