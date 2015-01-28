use std::io;

pub fn test() {
    println!("Try something!");


    let input = io::stdin().read_line().ok().expect("Failed to read line");

    println!("{}", input);
}
