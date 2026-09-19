struct rectangle{
    width: u32,
    height: u32,
}

impl rectangle{
    fn area(&self)-> u32{
    self.width * self.height
    }
    fn perimeter(&self)-> u32{
        self.width + self.height
    }
}

fn main(){
    let rect = rectangle{
        width: 12,
        height: 13,
    };
    println!("The area of the rectangle is {}", rect.area());
    println!("The perimeter of the rectangle is {}", rect.perimeter());
}