#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct Person {
    name: String,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("{}", self.name)
    }
}

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x1 = 5;
    let y1 = MyBox::new(x1);
    assert_eq!(5, x1);
    assert_eq!(5, *y1);

    let _p1 = Person {
        name: String::from("Glenn"),
    };
    let _p2 = Person {
        name: String::from("Joy"),
    };
}
