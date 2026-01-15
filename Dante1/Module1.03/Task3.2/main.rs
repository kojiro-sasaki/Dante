fn main(){
    let mut num: u32 = 0;
    loop{
        if num == 101{
            break;
        }
        println!("{num}");   
        num = num + 1;
    }
    return;
}