use std::io;

fn main() {
    println!("Enter your number\n");
    
    let mut input=String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    
    let num : i32 = input.trim().parse().unwrap();
    
    println!("Your number: {num} \nYour number*10: {}",num*10);
}