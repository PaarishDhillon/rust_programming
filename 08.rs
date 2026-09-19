fn main(){
    let x = String::from("Hello World");
    borrow(&x);
    println!("{}",x);
}

fn borrow(some: &String){
    println!("{}",some);
}