#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {

    let width1 = 30; 
    let height2 = 50; 

    let area1 = area(width1, height2);

    let rect = (30, 50);
    let area2 = areaTuple(rect);

    let rectS = Rectangle {
        width: 30, 
        height: 50,
    };

    let area3 = areaStruct(&rectS);


    println!(
        "The area of the rectangle is {} square pixles.",
        area3); 

    // if we want to print the struct: 

    //println!("{rectS:#?}");
    dbg!(&rectS);

    let testing = rectS.width;
    println!("{testing}");
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// using tuples

fn areaTuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


fn areaStruct(rectangle: &Rectangle) -> u32 {

    rectangle.height * rectangle.width
}