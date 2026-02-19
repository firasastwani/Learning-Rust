#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}



// defining methods (implementations ONTO specific types)

// this is different than a function, must always pass a refernce to the type as its first param
// these functions only work on the type they are specified to
// the are called using method syntax type.method() instead of function(type);
impl Rectangle {

    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    // associated functions that dont take &self as a param usually return Self and therefore are constructors 
    // for this, you use :: instead of .method()
    fn square(size: u32) -> Self {
        Self {
            width: size, 
            height: size,
        }
    }
}



fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // :: used for both namespace as well as associated functions (not methods)
    let sq = Rectangle::square(5);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
}
