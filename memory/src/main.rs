/*

fn main() {
    /*

    let mut s = String::from("hello");

    // let s = "duncan";

    s.push_str(", came home");

    println!("{s}");

    */

    /*
    let mut s = String::from("hello");

    s = String::from("different");

    let s1 = String::from("hello!");

    // this deletes s1,
    let s2 = s1;

    // this does not delete s1
    let s2 = s1.clone();


    // can copy INTS without using .clone();
    let x = 5;
    let y = x;

    */

    let s = String::from("Hello");

    takes_ownership(s.clone());

    println!("{s} this is SSSS");
}

// this function takes owernship of the string, must be passed as a colone if you want to keep it
fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
*/

/* 
fn main() {

    let s1 = gives_ownership();

    let s2 = String::from("hello"); 

    let s3 = takes_and_gives_back(s2); 

    println!("{s2}");
}




fn gives_ownership() -> String{

    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
*/

fn main() {

    let s1 = String::from("helo");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}


fn calculate_length(s: String) -> (String, usize) {

    let len = s.len();

    (s, len)
}