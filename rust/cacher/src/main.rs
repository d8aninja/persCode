use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value.contains_key(&arg) {
            true => {
                println!("Can't reuse this Cacher instance with {}.", arg);
                arg
            },
            false => {
                let val = (self.calculation)(arg);
                self.value.insert(arg, val);
                arg
            },
        }
    }
}

fn main() {
    let mut cache1 = Cacher::new(|x| x + 1 );
    cache1.value(1);
    dbg!(&cache1.value);
    cache1.value(1);
    dbg!(cache1.value);
}