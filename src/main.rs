use std::io::{self, Write};

fn main() {
    print!("Enter current stock price: ");
    io::stdout().flush().expect("Failed to flush output");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let price: f64 = match input.trim().parse::<f64>() {
        Ok(value) if value.is_finite() && value > 0.0 => value,
        _ => {
            eprintln!("Please enter a valid positive price.");
            return;
        }
    };

    let sqrt_price = price.sqrt();
    let base = sqrt_price.trunc() as i64;

    println!("\nCurrent Price        : {:.2}", price);
    println!("Square Root          : {:.4}", sqrt_price);
    println!("Truncated Square Root: {}", base);

    println!(
        "\n{:<10} {:<15} {:<15}",
        "Direction", "Root Level", "Gann Price"
    );
    println!("{}", "-".repeat(40));

    // Five Gann levels below the truncated square root
    for offset in (1..=5).rev() {
        let root = base - offset;
        let gann_price = make_odd(root * root);

        println!("{:<10} {:<15} {:.2}", "Below", root, gann_price);
    }

    // Reference level
    println!("{:<10} {:<15} {:.2}", "Base", base, make_odd(base * base));

    // Five Gann levels above the truncated square root
    for offset in 1..=5 {
        let root = base + offset;
        let gann_price = make_odd(root * root);

        println!("{:<10} {:<15} {:.2}", "Above", root, gann_price);
    }
}

// If the Gann price is even, add 1 to make it odd.
fn make_odd(price: i64) -> i64 {
    if price % 2 == 0 { price + 1 } else { price }
}
