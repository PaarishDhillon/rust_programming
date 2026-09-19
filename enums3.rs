use std::fs;

enum Result<A,B>{
    Ok(A),
    Err(B),
}

fn main(){
    let read = fs::read_to_string("notes.txt");
    println!("{:?}",read);
    match read{
        Ok(content)=>{
            println!("File content: {}",content);
        }
        Err(err)=>{
            println!("Error: {}",err);
        }
    }
    println!("Hello world");
}