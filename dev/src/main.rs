use dev::{Tweet, Summary};


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
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}
