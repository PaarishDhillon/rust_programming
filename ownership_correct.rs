fn main(){
    let mut my_string = String::from("Hello");
    my_string = take_string(my_string);
    println!("{}",my_string);
}

fn take_string(some:String) -> String{
    println!("{}",some);
    return some;
}