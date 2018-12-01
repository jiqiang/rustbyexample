enum List {
    Cons(i32, Rc<List>),
    Nil
}

use List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("{}", Rc::strong_count(&a));
    let _b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("{}", Rc::strong_count(&a));
    {
        let _c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("{}", Rc::strong_count(&a));
    }
    println!("{}", Rc::strong_count(&a));
}
