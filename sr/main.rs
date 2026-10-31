use std::io::{self, Write};

fn main() {
    println!("Rust Calculator — enter an expression like 12 * 3, or q to quit.");

    loop {
        print!("> ");
        io::stdout().flush().expect("Could not flush prompt");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Could not read input.");
            continue;
        }

        let input = input.trim();

        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 3 {
            println!("Use this format: number operator number");
            continue;
        }

        let left: f64 = match parts[0].parse() {
            Ok(number) => number,
            Err(_) => {
                println!("That first value isn't a number.");
                continue;
            }
        };

        let right: f64 = match parts[2].parse() {
            Ok(number) => number,
            Err(_) => {
                println!("That second value isn't a number.");
                continue;
            }
        };

        let result = match parts[1] {
            "+" => Some(left + right),
            "-" => Some(left - right),
            "*" => Some(left * right),
            "/" if right != 0.0 => Some(left / right),
            "/" => {
                println!("Can't divide by zero.");
                continue;
            }
            _ => {
                println!("Unknown operator. Use +, -, *, or /.");
                continue;
            }
        };

        println!("{}", result.unwrap());
    }

    println!("Goodbye!");
}
