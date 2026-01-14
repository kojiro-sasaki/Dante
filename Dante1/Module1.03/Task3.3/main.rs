fn main() {
 println!("Print sum");
 let mut sum:String = String::new();
 std::io::stdin().read_line(&mut sum).expect("Failed to read line");
 let mut sum:f64=sum.trim().parse().expect("Print number");
 if sum < 0.0{
     println!("Print correct sum");
 }
 println!("Print range");
 let mut range:String = String::new();
 std::io::stdin().read_line(&mut range).expect("Failed to read line");
 let range:i32=range.trim().parse().expect("Print number");
 for _ in  0..range{
     sum=sum+sum*0.01;
 }
 println!("Total sum = {:.2}",sum);
}