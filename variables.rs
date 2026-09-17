// overflow state

fn main(){
    let mut x: i8 = 120;

    for i in 1..10000 {
        x+=1;
    }

    println!("{}", x);
}