use std::fs;

fn main(){
    let res = read("notes.txt".to_string());
    println!("{}",res);
    println!("Hello World");
}

fn read(file_content: String)-> String{
    let res = fs::read_to_string("notes.txt");
    return res.unwrap();
}