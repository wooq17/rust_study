use std::io;
use std::io::Write;

extern crate rand;

const INPUT_NUMBER_COUNT: usize = 3;
const TURN_LIMIT: u32 = 9;

enum TurnResult {
	Cont(u32, u32),
	Retry(&'static str),
}

fn generate_answer() -> Vec<u32> {
	let mut answer: Vec<u32> = Vec::new();

	while answer.len() < INPUT_NUMBER_COUNT {
		let rand_number = rand::random::<u32>() % 9 + 1;

		match answer.iter().find(|x| **x == rand_number) {
			Some(_) => {},
			None => answer.push(rand_number),
		}
	}

	answer
}

fn convert_input(input: String) -> Result<Vec<u32>, &'static str> {
	let input_numbers: Vec<&str> = input.trim().split(' ').collect();

	if input_numbers.len() == INPUT_NUMBER_COUNT {	
		let mut retun_val:Vec<u32> = Vec::new();

		for each_number_str in input_numbers {
			// each input number validation
			match u32::from_str_radix(each_number_str, 10) {
				Ok(num) => { retun_val.push(num); },
				Err(_) => { return Err("invalid number format"); },
			};
		}

		Ok(retun_val)
	} else {
		Err("invalid input number")
	}
}

fn progress_turn(answer: &Vec<u32>) -> TurnResult {
	let mut input: String = String::new();
	io::stdin().read_line(&mut input);

	// input format validation
	let input_numbers = match convert_input(input) {
		Ok(numbers) => numbers,
		Err(message) => {
			return TurnResult::Retry(message);
		}
	};

	let mut strike = 0;
	let mut ball = 0;

	// compare with answer
	for (idx, each_number) in input_numbers.iter().enumerate() {
		match answer.iter().find(|x| **x == *each_number) {
			Some(_) => {
				if answer[idx] == *each_number { strike += 1; }
				else { ball += 1; }
			},
			None => {},
		}
	}

	TurnResult::Cont(strike, ball)
}

fn main() {
	io::stdout().flush();

	let mut turn = 1;

	let answer = generate_answer();

	println!("[DEBUG] answer : {:?}", answer);
	println!(">>> Game started...");

	loop {
		if turn > TURN_LIMIT {
			println!("game over...");
			return;
		}

		println!("[turn : {}] Input your answer", turn);

		let (strike, ball) = match progress_turn(&answer) {
			TurnResult::Cont(s, b) => (s, b),
			TurnResult::Retry(message) => {
				println!("{}", message);

				println!("check your input format");
				println!("input format : number number number");
				println!("example : 1 2 3");

				continue;
			},
		};

		println!("[turn : {}] strike : {}, ball : {}\n", turn, strike, ball);

		if strike == INPUT_NUMBER_COUNT as u32 {
			println!("clear!");
			println!("your score : {} turn", turn);
			return;
		}

		turn += 1;
	}
}