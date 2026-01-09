fn main() {
    println!("Enter your number:");
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: f32 = input.trim().parse().expect("Input is not a number");
    println!("Your number = {}", input);
    println!("Your number * 10 = {}", input * 10.0);
}
