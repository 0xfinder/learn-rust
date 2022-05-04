fn main() {
    println!("Hello, world!");

    twelve_days_of_christmas();
}

fn fibonacci(n: i32) -> i32 {
    if n < 2 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}

fn convert_to_celsius(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn convert_to_fahrenheit(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}

fn twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];
    let count = [
        "a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ];

    for i in 0..days.len() {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[i]
        );

        for j in (0..i + 1).rev() {
            println!("{} {}", count[j], gifts[j]);
        }

        if i == days.len() - 1 {
            break;
        }

        println!("\n{} {}", count[i + 1], gifts[i + 1]);
    }
}
