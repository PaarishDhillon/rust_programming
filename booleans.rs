fn main(){
    let is_male = true;
    let mut above_18 = true;
    above_18 = false;
    if is_male{
        println!("You are a male");
    }
    else{
        println!("You are not a male");
    }
    if is_male && above_18{
        println!("You are an adult male");
    }
}