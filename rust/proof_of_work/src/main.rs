fn main() {
    
    // messing with https://gankra.github.io/blah/linear-rust/
    // trying to demo the "proof of work" assertion by the author
    // #[derive(Clone)] // uncomment to fiddle
    struct Step1Token { field: u8};
    // #[derive(Clone)] //uncomment to fiddle
    struct Step2Token { field: u8};

    fn step1() -> Step1Token { Step1Token{field: 1} };                // malloc, fopen, glCreateContext, connect
    fn step2(_t: Step1Token) -> Step2Token { Step2Token{field: 2} };   // write, read, mutate, draw, send
    fn step3(_t: Step2Token) {println!("Finished!")};                             // free, close, destroy

    let token1 = step1();
    let token2 = step2(token1); // token1.clone() ... but how does that undermine the proof of work litmus?
    step3(token2);
    // step2(token1); // ERROR: use of moved value, does not implement Copy / is not Copy
    // step3(token2); // ERROR: use of moved value, does not implement Copy / is not Copy

}
