use std::collections::HashMap;

fn main() {

    //let mut v: Vec<i32> = Vec::new();

    //let v = vec![1,2,3];

    /* 
    v.push(5);
    v.push(6);
    v.push(7);
    */

    let v = vec![1, 2, 3, 4, 5];

    /* 
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    */


    let first = &v[0];

    v.push(1);

    println!("The first element is {first}");


    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10); 
    scores.insert(String::from("yellow"), 50);

    let team_name = String::from("blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);
    

}
