use std::collections::HashMap;
fn main() {
    hashmap();
}

fn _vector() {
    let mut v: Vec<i32> = Vec::new();

    let _v1 = vec![1, 2, 3, 4, 5];

    v.push(4);
    v.push(3);
    v.push(6);
    v.push(9);

    v.pop();

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element iz {}", third),
        None => println!("there is no third element"),
    }

    // let _does_not_exist = &v[100];
    let _does_not_exist = v.get(100);

    println!("{:#?}", _does_not_exist);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    println!("{:#?}", v);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.22),
        SpreadsheetCell::Text(String::from("hello wordl")),
    ];
}

fn _string() {
    let mut _s = String::new();

    let data = "initial contents";
    let mut s = data.to_string(); // String::from(data)

    s.push_str(&data[..]);

    println!("data is {}", s);

    let mut lo = String::from("lo");

    lo.push('l');
    println!("{}", lo);

    let s1 = String::from("hello, ");
    let s2 = String::from("world!");
    // deref coercion
    // can only add &str to String
    // rust turns &s2 of type &String into &s2[..] of type &str
    // ownership of s1 is taken
    let s3 = s1 + &s2;

    let s = format!("{}{}", s2, s3);
    println!("{}", s);
}

fn hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];

    // let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    scores.entry(String::from("Black")).or_insert(69);

    for (key, value) in &scores {
        println!("{}: {}", key, &value);
    }
}
