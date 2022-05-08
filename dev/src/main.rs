fn main() {
    vector();
}

fn vector() {
    let mut v: Vec<i32> = Vec::new();

    let _v1 = vec![1, 2, 3, 4, 5];

    v.push(4);
    v.push(3);
    v.push(6);
    v.push(9);

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
