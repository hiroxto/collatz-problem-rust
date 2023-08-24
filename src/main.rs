use std::env;

fn collatz(n: u32, count: u32) -> (u32, u32) {
    if n == 1 {
        return (n, count);
    }

    let next = match n % 2 {
        0 => n / 2,
        _ => 3 * n + 1,
    };
    return collatz(next, count + 1);
}

fn start_collatz(n: i32) -> Result<(u32, u32), String> {
    if n >= 1 {
        Ok(collatz(n as u32, 1))
    } else {
        Err(String::from("数字には1以上を指定してください。"))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_number:i32= args[1].parse().unwrap();

    let result = start_collatz(input_number);

    match result {
        Ok(v) => println!("指定された値: {}, 計算結果: {}, 回数: {}回", input_number, v.0, v.1),
        Err(e) => println!("Error!: {}", e),
    }
}
