fn main() {
    println!("Hello, world!");

    let mut x = 3; 

    x = another_function(x);

    if x == 3 {
        println!("You need to pass it as a reference");
    } else {
        println!("Changed the value by PASSING as reference, then dereferencing in the function!")
    }
}


fn another_function(x: i32) -> i32{
    x + 3
}