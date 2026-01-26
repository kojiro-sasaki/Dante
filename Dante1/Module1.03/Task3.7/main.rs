fn main() {
    let mut pow: i128 = 1;
    for i in 0..20 {
        println!("{} {}", i, pow);
        pow = pow * 10;
    }
