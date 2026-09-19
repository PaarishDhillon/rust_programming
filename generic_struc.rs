struct Point<A,B,C>{
    x: A,
    y: B,
    z: C,
}

fn main(){
    let pointer = Point{x:5, y:"Paarish", z:2.6};
    println!("{},{},{}",pointer.x, pointer.y, pointer.z);
}