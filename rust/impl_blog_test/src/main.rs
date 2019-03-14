//https://tratt.net/laurie/blog/entries/a_quick_look_at_trait_objects_in_rust.html
trait T {
  fn m(&self) -> u64;
}

struct S {
  i: u64
}

impl T for S {
  fn m(&self) -> u64 { self.i }
}

struct S1 {
  i: u64
}

impl T for S1 {
  fn m(&self) -> u64 { self.i * 2 }
}

struct S2 {
  j: u64
}

impl T for S2 {
  fn m(&self) -> u64 { self.j * 4 }
}

fn f(x: &T) {
  println!("{}", x.m())
}

fn f2(x: Box<T>) {
  println!("{}", x.m())
}

fn main() {
  let s = S{i : 100};
  println!("{}", s.m());

  let s1 = S1{i : 100};
  f(&s1);
  let s2 = S2{j : 100};
  f(&s2);

  let b: Box<S1> = Box::new(S1{i: 100});
  f2(b);
}