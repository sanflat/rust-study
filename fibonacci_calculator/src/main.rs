use std::io;

fn main() {
    println!("フィボナッチ数列のn番目を求めます。nを入力してください:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input.trim().parse().expect("入力された値は整数でなければなりません");

    let fib_number = fibonacci(n);
    println!("フィボナッチ数列の第{}項は{}です", n, fib_number);
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
