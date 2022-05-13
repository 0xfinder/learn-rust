use std::collections::HashMap;

// Chapter 8
fn main() {
    let list = [5, 12, 6, 7, 13, 0, 7, 14, 10, 8, 7, 1, 3];

    let mut xs: Vec<i32> = list.into_iter().collect();

    println!("{:?}", &xs);

    println!("Mean: {:?}", mean(&xs));

    println!("Median: {:?}", median(&mut xs));

    println!("Mode: {:?}", mode(&xs));

    let text = String::from("anarchy");
    println!("{} in pig latin: {}", text, pigify(&text[..]))
}

fn mean(ints: &Vec<i32>) -> f32 {
    let sum: i32 = ints.iter().sum();

    sum as f32 / ints.len() as f32
}

fn median(ints: &mut Vec<i32>) -> f32 {
    ints.sort();
    let length = ints.len();
    if length % 2 == 0 {
        (ints[length / 2] + ints[(length / 2) - 1]) as f32 / 2 as f32
    } else {
        ints[length / 2] as f32
    }
}

fn mode(ints: &Vec<i32>) -> i32 {
    let mut occurrences: HashMap<i32, i32> = HashMap::new();

    for &value in ints {
        let count = occurrences.entry(value).or_insert(0);
        *count += 1;
    }

    occurrences
        .into_iter()
        .max_by_key(|&(_, value)| value)
        .map(|(val, _)| val)
        .expect("No numbers in list")
}

fn pigify(text: &str) -> String {
    let mut letters = text.chars();
    let first_letter = letters.next().unwrap();
    let first_letter = first_letter.to_lowercase().next().unwrap();

    match first_letter {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", text),
        _ => format!("{}-{}ay", letters.as_str(), first_letter),
    }
}
