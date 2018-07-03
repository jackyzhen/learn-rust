use std::ops::Deref;
use std::rc::Rc;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// impl Deref so box vlaue can be deferenced with *
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

enum List {
    // use box to break recursive struct size resolution
    // forces allocation on heap
    Cons(i32, Box<List>),
    Nil,
}

enum Rc_List {
    Rc_Cons(i32, Rc<Rc_List>),
    Rc_Nil,
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

use List::{Cons, Nil};
use Rc_List::{Rc_Cons, Rc_Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // demonstrating rust auto recursive deref.
    // deref() call on &MyBox<String> to &String then deref to &str
    let z = MyBox::new("Jacky".to_string());
    hello(&z);

    // equ to this (deref z (MyBox<String>) to String, then &[..] to take slice of String
    hello(&(*z)[..]);

    // multi ownership

    let a = Rc::new(Rc_Cons(5, Rc::new(Rc_Cons(10, Rc::new(Rc_Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc_Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Rc_Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a))
}
