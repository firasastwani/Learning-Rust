fn main() {

    let mut s1 = String::from("hello");

    // value of s1 is not dropped here because its being passes as a reference. 
    let len = calculate_length(&mut s1);

    println!("The length of '{s1}' is {len}.");

    //let s = dangle();   

    let s = dont_dangle();  
    println!("{s}")
}

// cannot make changes to this as its an IMMUTABLE reference by default, breaks
fn calculate_length(s: &mut String) -> usize{

    s.push_str(", world");

    s.len()
}


// returning this reference to s breaks the code because s goes out of scope at the end of the function
// this creates a dangling reference which is not allowed in rust 
/* 
fn dangle() -> &String {

    let s = String::from("hello");

    &s
}
*/


// this does not create a dangling refernce because ownership of this spot of memory is passed directly
// from the function!
fn dont_dangle() -> String {

    let s = String::from("hello");

    s
}