fn main(){
    let mut variable = 12;
    println!("{}", variable);
    // variable = 32;
    // println!("{}", variable);
    while variable < 15{
        variable += 1;
    }
    println!("{}", variable);
    let result = sum(12, 32);
    println!("Sum = {}", result);
}

pub fn sum(a: i32, b: i8) -> i32{
    return a + b as i32;
}