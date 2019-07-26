fn main() {
    use codegen::Scope;

    let mut scope = Scope::new();

    scope.new_struct("Foo")
        .derive("Debug")
        .field("one", "usize")
        .field("two", "String");

    println!("{}", scope.to_string());
}
