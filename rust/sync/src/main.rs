use std::thread;
use rand::Rng;

fn get_two_nums() {
    // there is an a/sync version of this function example at
    // https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html
    // but it is not 'real'
    let thread_one = thread::spawn(|| {
        let mut rng = rand::thread_rng();
        println!("Integer: {}", rng.gen_range(0, 10));
    });

    let thread_two = thread::spawn(|| {
        let mut rng = rand::thread_rng();
        println!("Float: {}", rng.gen_range(0.0, 10.0))
    });

    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

fn main() {
    get_two_nums();
}
