fn main() {

    //Using primitive values
    // let width1 = 30;
    // let height1 = 50;
    // println!("The area of the rectangle is: {} sq. pixels",
    //  area(width1, height1));


    //Using tuples.
    // let rectangle = (30,50);
    // println!("The area of the rectangle is: {} sq. pixels",
    // area(rectangle));


    //Using structs?
    let width = 30;
    let height = 50;
    let rectangle = Rectangle{width, height};

    println!("The area of the rectangle is: {} sq. pixels",
    rectangle.area());

    let rect2 = Rectangle{
        width:20, height:30
    };
    println!("rectangle can hold rect2? {}", rectangle.can_hold(&rect2));
    println!("rect2 can hold rectangle? {}", rect2.can_hold(&rectangle));
}
struct Rectangle{
    width:u32,
    height:u32
}

//Using primitive values
// fn area(width:u32, height:u32)->u32{
//     width * height
// }


//Using primitive values
// fn area (dimensions: (u32, u32))->u32{
//     dimensions.0 * dimensions.1
// }


//Using structs
impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
    
    fn can_hold(&self, other : &Rectangle)->bool{
        self.width >= other.width && self.height>=other.height
    }

    fn square(size: u32)->Self{
        Self { width: size, height: size }

    }
}