

// 预期写一个遍历目录的  蒙圈了

use walkdir::WalkDir;

trait PathWalkFn {
    fn walk_fn(&mut self, path0 :&std::path::Path);
}

struct PathStat {
    count :i64
}

fn walk<T:PathWalkFn>(root :&std::path::Path, walk_fn_obj: &mut T)   {
    for result_entry in WalkDir::new(root).follow_links(true) {
        match result_entry {
            Ok(valid) => {
                walk_fn_obj.walk_fn(valid.path());
            },
            Err(err) => {
                println!("ERROR {:?}", err);
            },
        };
    }
}


impl PathWalkFn for PathStat {
    fn walk_fn(&mut self, _path :&std::path::Path) {
        self.count += 1;
    }
}


fn main() {

    let mut walk_fn = PathStat{count:0};
    let root = String::from(".");
    let root_path  = std::path::Path::new(&root);

    let root_path1 =root_path.canonicalize();
    match root_path1 {
        Ok(root_path2)=> {
            println!("enum directory {}", root_path2.display());
            walk(&root_path2, &mut walk_fn);
        },
        Err(err) => {
          println!("get err= {}", err);
        },
    }
    println!("wo get {} files", walk_fn.count);
    println!("main exit");
}
