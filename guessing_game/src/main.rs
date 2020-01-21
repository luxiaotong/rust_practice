use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess!");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed");
        println!("Your guess is {}", number);

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&secret_number) {
            Ordering::Less      => println!("Small"),
            Ordering::Greater   => println!("Big"),
            Ordering::Equal     => {
                println!("Win");
                break;
            },
        }
    }
}
