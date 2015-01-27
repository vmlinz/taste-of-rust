use std::cmp::Ordering;

pub fn my_tuples() {
    let x = (1, "hello");

    let y: (i32, &str) = (2, "world");

    let (x1, x2) = (1, 2);

    println!("x1 = {}, x2 = {}", x1, x2);
}

pub fn my_structs() {
    struct Point {
        x: i32,
        y: i32,
    };
    let origin = Point {x: 0, y: 0};

    println!("The origin is at ({}, {})", origin.x, origin.y);

    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    struct Inchies(i32);
    let Inchies(len) = Inchies(10);
}

fn cmp(a: i32, b: i32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}

pub fn my_enums() {
    let (x, y) = (5, 6);
    if cmp(x, y) == Ordering::Less { 
        println!("less");
    }        
}

