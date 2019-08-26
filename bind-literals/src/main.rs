
fn add_one(n :&mut i64) {
    *n += 1;
}

fn main() {

    let mut num = 3;

    println!("num= {}", num);

    add_one(&mut num);

    println!("num= {}", num);

    // this can success, while in c++ will got `SIGSEV`
    add_one(&mut 0);

    println!("Hello, world!");
}
