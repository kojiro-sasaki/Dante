fn main() {
    println!("Enter your first number:");

    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read");

    println!("Enter your second number:");

    let mut number2 = String::new();
    std::io::stdin().read_line(&mut number2).expect("Failed to read");

    let number: f32 = number.trim().parse().expect("Invalid number");
    let number2: f32 = number2.trim().parse().expect("Invalid number");

    println!("Sum: {}", number + number2);
    println!("Diff: {}", number - number2);
    println!("Product: {}", number * number2);

    if number2 != 0.0 {
        println!("Div: {}", number / number2);
    } else {
        println!("Div: division by zero!");
    }
}
