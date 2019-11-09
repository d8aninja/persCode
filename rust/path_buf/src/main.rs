use std::path::{PathBuf, Path};

fn main() {

//    for i in ["/Users", "jeffvangeete", "git"].iter() {
//        println!("{}", i);
//    }

    let path: PathBuf = ["/Users", "jeffvangeete", "git"].iter().collect();

    dbg!(path);

}