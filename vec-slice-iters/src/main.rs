fn vec_iter() {
    let mut a = vec![String::from("a"), String::from("b"), String::from("c")];

    println!("a={:?}", a);

    for v in a.iter() {
        //     found type `&std::string::String`
        println!("v={:?}", v);
    }
    println!("a={:?}", a);
}

fn vec_iter_mut() {
    let mut a = vec![String::from("a"), String::from("b"), String::from("c")];

    println!("a={:?}", a);

    for v in a.iter_mut() {
        // &mut std::string::String
        println!("v={:?}", v);
    }
    println!("a={:?}", a);
}

fn vec_into_iter() {
    let mut a = vec![String::from("a"), String::from("b"), String::from("c")];

    println!("a={:?}", a);

    for v in a.into_iter() {
        // std::string::String
        println!("v={:?}", v);
    }
    // cannot borrow here
    // println!("a={:?}", a);
}

fn vec_into_iter_borrow() {
    let mut a = vec![String::from("a"), String::from("b"), String::from("c")];

    println!("a={:?}", a);

    // std::vec::IntoIter<std::string::String>
    let mut a_into_iter = a.into_iter();
    println!("into iter={:?}", a_into_iter);

    for v in a_into_iter {
        // std::string::String
        println!("v={:?}", v);
    }
    // cannot borrow here
    //println!("into iter={:?}", a_into_iter);
}

fn vec_for() {
    let mut a = vec![String::from("a"), String::from("b"), String::from("c")];

    println!("a={:?}", a);

    for v in a {
        // std::string::String
        println!("v={:?}", v);
    }
    // cannot borrow, borrow of moved value: `a`
    // println!("a={:?}", a);
}

fn vec_for_borrow() {
    let mut a = vec![String::from("a"), String::from("b"), String::from("c")];

    println!("a={:?}", a);

    for v in &a {
        // &std::string::String
        println!("v={:?}", v);
    }
    println!("a={:?}", a);
}

fn vec_for_mut_borrow() {
    let mut a = vec![String::from("a"), String::from("b"), String::from("c")];

    println!("a={:?}", a);

    for v in &mut a {
        // &mut std::string::String
        println!("v={:?}", v);
    }
    println!("a={:?}", a);
}

fn slice_iter() {
    let mut a = [String::from("a"), String::from("b"), String::from("c")];
    println!("a={:?}", a);

    for v in a.iter() {
        // `&std::string::String`
        println!("v={:?}", v);
    }
    println!("a={:?}", a);
}

fn slice_iter_mut() {
    let mut a = [String::from("a"), String::from("b"), String::from("c")];
    println!("a={:?}", a);

    for v in a.iter_mut() {
        // `&mut std::string::String`
        println!("v={:?}", v);
    }
    println!("a={:?}", a);
}

fn slice_into_iter() {
    let mut a = [String::from("a"), String::from("b"), String::from("c")];
    println!("a={:?}", a);

    // no this method
    println!("a={:?}", a);
}

fn slice_for() {
    let mut a = [String::from("a"), String::from("b"), String::from("c")];
    println!("a={:?}", a);

    // borrow the array with `&` or call `.iter()` on it to iterate over it
    // [std::string::String; 3]` is not an iterator
    //for v in a {
    //    println!("v={:?}", v);
    //}
    println!("a={:?}", a);
}

fn slice_for_borrow() {
    let mut a = [String::from("a"), String::from("b"), String::from("c")];
    println!("a={:?}", a);

    for v in &a {
        // &std::string::String`
        println!("v={:?}", v);
    }
    println!("a={:?}", a);
}
fn slice_for_mut_borrow() {
    let mut a = [String::from("a"), String::from("b"), String::from("c")];
    println!("a={:?}", a);

    for v in &mut a {
        // &mut std::string::String`
        println!("v={:?}", v);
    }
    println!("a={:?}", a);
}
fn main() {
    vec_into_iter_borrow();
}
