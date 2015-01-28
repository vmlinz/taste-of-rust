pub fn test() {
    for x in range(0, 10) {
        println!("{}", x); // x: i32
    }

    let mut x = 5u;
    let mut done = false;

    while !done {
        x += x - 3;
        println!("{}", x);
        if x % 5 == 0 { done = true; }
    }

    let mut y = 5u;
    loop {
        y += y -3;
        println!("{}", y);
        if y % 5 == 0 { break; }
    }
}

