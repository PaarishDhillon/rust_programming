fn main(){
    let mut x = String::from("Hello");
    println!("{}",x);
    let y = &mut x;
    y.push_str(" World");
    println!("{}",x);
}