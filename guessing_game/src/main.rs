use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("GUESSING GAME!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
//    println!("Your secret number is {secret_number}");

    loop{
        let mut guess = String::new();
        println!("Please guess the number: ");
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please input a number");

        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Secret Number is Greater than {guess}"),
            Ordering::Greater => println!("Secret Number is Less than {guess}"),
            Ordering::Equal => {
                println!("You got it Congrats 🎉🥳");
                break;
            }
        }
    }

}
