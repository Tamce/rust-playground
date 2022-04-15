use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("This programe has chosen a number between [1, 100], try guess it!");
    loop {
        let mut guess = String::new();
        println!("Input your guess:");
        io::stdin().read_line(&mut guess).expect("read line failed");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Not a number! {}", e);
                continue;
            },
        };
        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            }
            Ordering::Less => println!("Too small!"),
        }
    }
}
