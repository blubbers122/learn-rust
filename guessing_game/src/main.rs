use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	const MAX_GUESSES: u8 = 10;
	let mut guesses: u8 = 0;

	println!("Guess the number in under {} tries!", MAX_GUESSES);

	let secret_number: u32 = rand::thread_rng().gen_range(1..101);

	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please enter a number!");
				continue;
			}
		};

		println!("You guessed: {}", guess);
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}

		guesses += 1;
		if guesses == MAX_GUESSES {
			println!("You ran out of guesses! :(");
			break;
		}
	}
}
