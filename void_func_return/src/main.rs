

// 一些人用 `无返回类型无参数的函数` 来形容这样的函数， fn name() {}

// 我认为这是不合理的。在 Rust 中，除去发散函数，其他函数都有返回值，即使是

// 这样的函数，用下面例程验证。

fn void_func_return(){

    println!("a func no Explicit return value");
}


fn main() {

    let value1 = void_func_return();
    println!("value1={:?}", value1);

    let value2:() = void_func_return();
    println!("value2={:?}", value2);

    println!("main exit");
}

/*
value1=()
value2=()
*/
