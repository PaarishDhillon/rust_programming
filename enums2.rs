enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64,f64),
}

fn area(shape: Shape)-> f64{
    match shape{
        Shape::Circle(radius) => 3.14*radius*radius,
        Shape::Square(side) => side*side,
        Shape::Rectangle(height,width) =>{
            println!("Hi");
            height*width
        }
    }
}

fn main(){
    let Circle = Shape::Circle(2.8);
    let Square = Shape::Square(5.0);
    let Rectangle = Shape::Rectangle(2.0, 3.0);
    println!("Area of Circle: {}",area(Circle));
    println!("Area of Square: {}",area(Square));
    println!("Area of Rectangle: {}",area(Rectangle));
}