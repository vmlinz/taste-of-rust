pub fn test() {
    let string = "Hello there.";

    let mut s = "Hello".to_string();
    println!("{}", s);

    takes_slice(s.as_slice());
    compare(s);
    
}

fn takes_slice(slice: &str) {
    println!("Got: {}", slice);
}

fn compare(string: String) {
    if string.as_slice() == "Hello" {
        println!("yes");
    }
}

