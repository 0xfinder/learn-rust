fn main() {
    let n = 10;

    println!("{}th fibonacci number: {}", n, fibonacci(n - 1));
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
