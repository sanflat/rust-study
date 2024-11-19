use std::io;

fn main() {
    loop {
        println!("温度を入力してください (例: 25C, 77F):");
        let mut temp_input = String::new();

        io::stdin()
            .read_line(&mut temp_input)
            .expect("読み込みに失敗しました");

        let temp_input = temp_input.trim();
        if let Some(last_char) = temp_input.chars().last() {
            let value = &temp_input[..temp_input.len()-1];

            match value.parse::<f64>() {
                Ok(num) => {
                    match last_char {
                        'C' | 'c' => {
                            let fahrenheit = celsius_to_fahrenheit(num);
                            println!("摂氏 {}°C は 華氏 {}°F です", num, fahrenheit);
                            break;
                        },
                        'F' | 'f' => {
                            let celsius = fahrenheit_to_celsius(num);
                            println!("華氏 {}°F は 摂氏 {}°C です", num, celsius);
                            break;
                        },
                        _ => println!("不正な形式です。摂氏は 'C'、華氏は 'F' で終わる必要があります。"),
                    }
                },
                Err(_) => {
                    println!("数値の変換に失敗しました。正しい形式で入力してください。");
                    continue;
                }
            }
        } else {
            println!("入力が不正です。もう一度入力してください。");
            continue;
        }
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0/9.0
}