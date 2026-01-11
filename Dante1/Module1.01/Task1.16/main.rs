fn main() {
    println!("Enter A:");
    
    let mut a = String::new();
    std::io::stdin().read_line(&mut a).expect("Failed to read line");
    let a:f32=a.trim().parse().expect("Invalid number");
    
    println!("Enter B:");
    let mut b=String::new();
    std::io::stdin().read_line(&mut b).expect("Failed to read line");
    let b:f32=b.trim().parse().expect("Invalid number");
    
    println!("Enter C");
    let mut c=String::new();
    std::io::stdin().read_line(&mut c).expect("Failed to read line");
    let c:f32=c.trim().parse().expect("Invalid number");
    
    let result:f32=(a.powi(2)+b.powi(2)+c.powi(2)).sqrt();
    println!("Result : {:.2}",result);
}
