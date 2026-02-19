// slices are a REFERNCE to a contiguous collection of elements

fn main() {

    let mut s = String::from("hello world");

    let n = first_word(&s);
    
    // now that s has been cleared, the value in n is USELESS
    //s.clear(); 

    // to fix this issue, we can take the slice immediatly in one line and hold that value, no matter what happens to s

    // this makes a slice which refernces a part of the string
    let hello = &s[0..5]; 
    let world = &s[6..11];

    // this will make an error because you now have a string slice pointing to this part of memory
    // so this mutable function will BREAK that immutable idea  
    //s.clear();

}

// &str is what signififies a "string slice"
// string literals are immutable, cannot change the contents of their refernce.

// instead of passing a &String here, its better to pass a &str because even if we have an &String we can just pass it as a slice and it works
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes(); 

    // enumerate returns a tuple of (index, item) so the for has to accept that with (i, &item)
    // enumerate also specifaclly gives a REFERNCE to that element
    for (i, &item) in bytes.iter().enumerate() {

        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}