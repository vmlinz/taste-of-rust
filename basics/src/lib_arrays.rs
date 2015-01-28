pub fn test() {
    let a = [1, 2, 3];
    println!("a has {} elements", a.len());

    for e in a.iter() {
        println!("{}", e);
    }


    println!("The second number is: {}", a[1]);

    let mut v = vec![1, 2, 3];

    v.push(4);
    println!("The length of v is {}", v.len());

    let middle = a.slice(1, 3);

    for e in middle.iter() {
        println!("{}", e);
    }
}
