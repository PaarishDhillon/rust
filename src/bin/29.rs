#[derive(Debug)]

struct rectangle{
    width: u32,
    height: u32,
}

fn main(){
    let rect1 = rect_build(12, 15);
    let rect2 = rect_build(10, 8);
    let rect3 = rect_build(11, 6);
    let rect4 = rect_build(12, 8);
    let sq = rectangle::square(3);
    println!("The area of the rectangle is: {}",(&rect1).area());
    println!("The area of the square is: {}",(&sq).area());

    println!("Can second rectangle fit in first: {}",&rect1.can_hold(&rect2));
    println!("Can fourth rectangle fit in third: {}",&rect3.can_hold(&rect4));
    println!("The rectangle is {rect1:?}");
}


fn rect_build(width:u32, height:u32) -> rectangle{
    rectangle{
        width,
        height,
    }
}
impl rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &rectangle) -> bool{
        self.area() > other.area()
    }

    fn square(size: u32) -> rectangle{
        rectangle{
        width: size,
        height: size,
        }
    }
}