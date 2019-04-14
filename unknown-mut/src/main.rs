
/**
// https://stackoverflow.com/questions/28587698/whats-the-difference-between-placing-mut-before-a-variable-name-and-after-the
// Rust          C/C++
    a: &T     == const T* const a; // can't mutate either
mut a: &T     == const T* a;       // can't mutate what is pointed to
    a: &mut T == T* const a;       // can't mutate pointer
mut a: &mut T == T* a;             // can mutate both

*/

/**


```rust
struct Point {
  a :i32,
  b :i32,
}
let a = &mut Point{a:1,b:2};
let b = &mut a; // error
```
如果理解 let 等式的左边称作绑定，等式右边称作对内存的访问权限控制。

`let a =` 是一个死绑定，不允许继续把 a 绑定到其他的内存，在这里就是其他的 Point

`let mut a = ` 是一个可变绑定，允许把 a 绑定到其他的 Point

`= &` 是对内存的只读借用，不允许修改内存

`= &mut` 是对内存的可修改借用，允许修改内存.

`=` 是转交所有权

那么基于以上规则，a 是可修改内存的绑定，b 也是可修改内存的绑定，

可为什么就编译错误了呢？
cannot borrow `a` as mutable, as it is not declared as mutable
这个语法不符合推理

*/

#[derive(Debug)]
struct Tupple{
    a :i32,
    b:i32,
}
fn different_use_mut(){
    let a = & Tupple{a:1,b:2};
    println!("a <{:?}> = &2;\n", a);
    let b = &mut Tupple{a:1,b:2};
    println!("b <{:?}> = &mut 2;\n", b);
    let mut c = Tupple{a:1,b:2};
    println!("mut c <{:?}> = 2;\n", c);
    //
}

fn mut_is_bind_0(){

    // here is a mut ref of temp value
    let mut a = &mut Tupple{a:1, b:2};

    println!("define a = {:?}", a);
    let b = &mut a;
    (b).a = 11;
    println!("a= {:?}", a);
}
fn mut_is_bind_1(){
    // here make a copy
    let mut a = Tupple{a:1, b:2};

    println!("define a = {:?}", a);
    let b = &mut a; // 为什么这里要用b修改内存时，必须要 a是mut的？
    (b).a = 11;
    a.a = 33; // a b 能同时修改
    println!("a= {:?}", a);
}

fn mut_is_bind_2(){
    let mut a = Tupple{a:1, b:2};

    println!("define a = {:?}", a);
    {
        let b = &mut a;
        (b).a = 11;
    }
    println!("a= {:?}", a);
}

fn mut_is_bind_3(){
    let mut a = &mut Tupple{a:1, b:2};
    println!("define a = {:?}", a);
    let b = &mut a;
    (b).a = 11;
    println!("a= {:?}", a);
}


fn main() {

    let _ = different_use_mut;
    let _ = mut_is_bind_0();
    let _ = mut_is_bind_1();
    let _ = mut_is_bind_2();
    let _ = mut_is_bind_3();


    println!("main exit");
}
