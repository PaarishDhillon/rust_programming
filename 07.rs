fn main(){
    let mut x = String::from("Hello");
    x = own(x);
    println!("{}",x);
}

fn own(some:String)->String{
    println!("{}",some);
    return some;
}