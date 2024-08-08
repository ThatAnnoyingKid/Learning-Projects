use std::io;
use rand::Rng;

fn main() {
	loop {
		println!("What is the lower number?");
		let mut lowIn = String::new();
		io::stdin().read_line(&mut lowIn).expect("Failed to read low line");
		let lowIn = lowIn.trim();
		let low: i32 = match lowIn.parse() {
			Ok(n) => n, 
			Err(e) => {
				println!("Failed to parse integer: {}", e);
				return;
			}
		};
	
		if low < 0 {
			println!("number is too low");
			continue; //used continue to keep low and high in the same scope
		}	
		
		println!("What is the higher number?");
		let mut highIn = String::new();
		io::stdin().read_line(&mut highIn).expect("Failed to read high number");
		let highIn = highIn.trim();
		let high: i32 = match highIn.parse() {
			Ok(n) => n,
			Err(e) => {
				println!("Failed to parse integer: {}", e);
				return;
			}
		};
		
		if high < low {
			println!("number is too low");
			continue;
		}
		//i want it to be known that im not proud of putting it all in one loop to keep everything in scope
		let mut rng = rand::thread_rng();
		let num: i32 = rng.gen_range(low..high + 1);
		println!("The range is between {} and {}", low, high);
		let mut guesses = 5;
		
		loop { //this project is disgusting
			println!("You have {} guesses left", guesses);
			let mut userGuessIn = String::new();
			io::stdin().read_line(&mut userGuessIn).expect("Failed to read low line");
			let userGuessIn = userGuessIn.trim();
			let userGuess: i32 = match userGuessIn.parse() { //i really should make this a function
				Ok(n) => n, 
				Err(e) => {
					println!("Failed to parse integer: {}", e);
					return;
				}
			};
			guesses -= 1;
			
			if userGuess > num {
				println!("Your guess is too high");
			} else if userGuess < num {
				println!("Your guess is too low");
			} else {
				println!("You guessed the number");
				break;
			}
			
			if guesses == 0 {
				println!("you did not guess the number");
				println!("The number was: {}", num);
				break;
			}
		}
		break;
	}
}
