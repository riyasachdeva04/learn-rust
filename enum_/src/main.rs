enum Shape{
    Rectangle(f64, f64),
    Square(f64),
    Circle(f64)
}

fn main(){
    let new_shape = Shape::Rectangle(4.0, 5.0);
    let area = calculate_area(new_shape);
    println!("{}", area);
}
fn calculate_area(shape: Shape) -> (f64){
    let area = match shape{
        Shape::Rectangle(a, b) => a*b,
        Shape::Circle(r) => 3.14*r*r,
        Shape::Square(a) => a*a
    };
    return area;
}
