use std::ops::Deref;
use std::mem::drop;

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
    
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    
    let c = CustomSmartPointer {
        data: String::from("stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c);
    println!("CustomSmartPointers created.");
}

fn hello(name: &str) {
    println!("hello, {}!", name);
}
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom smart pointer with data `{}`!", self.data);
    }
}

