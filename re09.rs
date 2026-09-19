fn main(){
    let mut x = String::from("Hello");
    println!("{}",x);
    update(&mut x);
    println!("{}",x);
}

fn update(some: &mut String){
    some.push_str(" World");
}