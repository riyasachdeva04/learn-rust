struct Rect{
    width: i32,
    height: i32
}

impl Rect{
    fn area(&self) -> i32{
        self.width * self.height
    }
}

fn main(){
    let r1 = Rect{
        width: 2,
        height: 5
    };
    println!("Area of rectangle is {}", r1.area());
}