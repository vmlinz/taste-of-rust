use std::io;
use std::rand;
use std::cmp::Ordering;

fn main() {
    println!("Let's play this guess game!");

    let secret = (rand::random::<uint>() % 100) + 1u;

    println!("The secret number is: {}", secret);

    loop {
        println!("Please input your guess.");

        let input = io::stdin().read_line().ok().expect("Failed to read line");

        let input_num: Option<uint> = input.trim().parse();

        let num = match input_num {
            Some(num) => num,
            None => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {}", num);

        match cmp(num, secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win, fuck!");
                return;
            }
        }
    }
}

fn cmp(a: uint, b: uint) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}
