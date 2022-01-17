use std::io;
use std::cmp::Ordering;
use rand::Rng;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 3 * 60;

fn main() {
    let mut x = 5 + THREE_HOURS_IN_SECONDS;
    let y = x = 6;

    let tup = (500, 2.1, 'c');

    println!("output the number, {}", x);

    println!("Guessing the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The serect number is {}", secret_number);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }

    another_function();
    let f = five();
    let f1 = plus_one(2);
}

fn another_function() {
    println!("Another function")
}

fn five() -> u32 {
    return 5;
}

fn plus_one(x: u32) -> u32 {
    return x + 1
}

