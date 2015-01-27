pub fn foo() {}

pub fn print_number(x: i32) {
    println!("x is {}", x);
}

pub fn add_one(x: i32) -> i32 {
    x + 1// return the expression
}

pub fn add_five(x: i32) -> i32 {
    if x < 5 { return x + 5;}

    x
}
