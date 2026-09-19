#[derive(Debug)]
enum direction{
    North,
    East,
    South,
    West,
}

fn main(){
    let my_dir = direction::North;
    println!("{:?}",my_dir);
}