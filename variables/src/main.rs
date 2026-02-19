use std::io;

fn main(){

    let a = [1, 2, 3, 4, 5];

    println!("please enter an array index");

    let mut index = String::new(); 

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index is not a number"); 


    let element = match a.get(index) {
        Some(&value) => value,
        None => {
            println!("Error: Index out of bounds.");
            return;
        }
    };

    println!("The value of the elemnet at index {index} is: {element}");
}