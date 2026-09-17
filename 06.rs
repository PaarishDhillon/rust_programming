fn main(){
    let mut x = String::from("Hello");
    // println!("Before update: {}",x);
    println!("Capacity: {}, Length: {}, pointer: {:p}", x.capacity(), x.len(), x.as_ptr());
    for _i in 1..100{
    x.push_str(" World");
    // println!("After update: {}",x);
    println!("Capacity: {}, Length: {}, pointer: {:p}", x.capacity(), x.len(), x.as_ptr());
}
}