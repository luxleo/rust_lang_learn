use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main()
{
    println!("Start guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("secret number : {secret_number}");

    loop{
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("guess please");

        //shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
