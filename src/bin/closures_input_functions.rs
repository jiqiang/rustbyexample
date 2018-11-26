fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I am a function!");
}

fn main() {
    let closure = || println!("I am a closure!");
    call_me(closure);
    call_me(function);
}
