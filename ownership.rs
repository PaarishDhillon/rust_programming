fn main(){
    let my_string = String::from("Hello");
    take_string(my_string); // This gives compilation error as ownership is now transferred to the string: 'some'
    println!("{}",my_string);
}

fn take_string(some:String){
    println!("{}",some);
}