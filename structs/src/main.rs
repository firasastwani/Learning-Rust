struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


// to use this shorthand notation, the params must have the same name as the trait!
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, 
        email,
        sign_in_count: 1,
    }
}

fn main() {

    // instances of structs can be made mutable
    let mut user1 = User {
        active: true,
        username: String::from("someuser@123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");

    // must pass as a refernce if you want it to keep owernship
    let mail = &user1.email;     
    println!("{mail}"); 

    let user2 = build_user(String::from("somebs@gmail.com"), String::from("usertest"));

    let m2 = user2.email;
    println!("{m2}");

    // struct update syntax:
    // copies the rest of the traits from user1. 

    // this MOVES the data from user1, it DOES NOT copy it.
    // this means that user3 is now the owner of this data

    // BUT if we had made new username as well, then user1 WOULD still be alive because copyable types remain!
    let user3 = User {
        email: String::from("another@exap"),
        ..user1
    };

    // this does not work, the value is moved
    let user1sdata = user1.email; 

    
}
