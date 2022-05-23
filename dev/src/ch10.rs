use dev::{Summary, Tweet};
use std::fmt::{Debug, Display};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    println!("the largest number is {}", largest(&number_list));

    let tweet = Tweet {
        username: String::from("edewd"),
        content: String::from("hey"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet, {}", tweet.summarize());

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Dwadwadwadwadwadwadwa. dwadwasdwasdwasdwasdwasdawsdwasd");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// wtf is wrong w the formatting
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    5
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

fn lifetimes() {
    {
        // let r;

        // {
        //     let x = 5;
        //     r = &x;
        // }

        // println!("r: {}", r);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement!, {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
