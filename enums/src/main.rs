// can put any kind of data into these enum params (even structs or other enums)
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit, 
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body defined here
    }
}

enum Coin {
    Penny, 
    Nickle, 
    Dime,
    Quarter,
}

impl Coin {
    
    fn value_in_cents(&self) -> i8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}


fn main() {

    let four = IpAddrKind::V4(String::from("127.0.0.1")); 

    let m = Message::Write(String::from("hello"));
    m.call(); 


    // here its made into an option automatically because its callign "some";
    let some_number = Some(5);
    let some_char = Some('e');

    let x: i8 = 5; 
    let y: Option<i8> = Some(5);


    let value = match y {
        Some(v) => v, 
        None => panic!("y was none"),
    }; 

    // shorthand
    let value = y.expect("y was none");
    let value = y.unwrap_or(0);

    let sum = match y {
        Some(v) => v + x,
        None => panic!("why was none"),
    };

    let q: Coin = Coin::Quarter; 


    // the type of option needs to be specified
    let absent_number: Option<i32> = None; 
}
