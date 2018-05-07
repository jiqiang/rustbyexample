fn main() {
    let _immutable_variable = 1;
    let mut mutable_variable = 1;

    mutable_variable += 1;
    println!("{}", mutable_variable);

    //_immutable_variable += 1;
    println!("{}", _immutable_variable);
}
