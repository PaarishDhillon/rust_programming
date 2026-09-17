// fn println(num: i32){
//     println!("{}",num);
// }
// fn main(){
//     let x = 12;
//     println(x);
// }

fn main(){
    stack_fn();
    heap_fn();
}

fn stack_fn(){
    let a = 12;
    let b = 23;
    let c = a+b;
    println!("The sum of {} and {} is {}",a,b,c);
}

fn heap_fn(){
    let mut str = String::from("Hello ");
    println!("{}",str);
    str.push_str("World");
    println!("{}",str);
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}",s1,s2);
    println!("{}", combined);
}