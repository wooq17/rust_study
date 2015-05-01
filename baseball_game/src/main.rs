use std::io;
use std::io::Write;

extern crate rand;

static NUMBER_COUNT: u32 = 3;

struct Result {
	strike: u32,
	ball: u32,
}

enum ValidationResult {
	Accept([u32; 3]),
	Reject,
}

fn generate_answer() -> [u32; 3] {
	let mut answer = [0; 3];
	for each_idx in 0..3 {
		'select_random_number: loop {
			let rand_number = rand::random::<u32>() % 9 + 1;
			for each in &answer {
				if rand_number == *each { continue 'select_random_number; }
			}

			answer[each_idx] = rand_number;
			break;
		}
	}

	answer
}

fn convert_input(input: String) -> ValidationResult {
	let input_numbers: Vec<&str> = input.trim().split(' ').collect();

	let element_count = input_numbers.len();
	if element_count == 3 {	
		let mut retun_val = [0; 3];

		for (idx, each_number_str) in input_numbers.iter().enumerate() {
			let each_number = u32::from_str_radix(each_number_str, 10);

			// each input number validation
			let input_num = match each_number {
				Ok(num) => num,
				Err(_) => { return ValidationResult::Reject; }
			};

			retun_val[idx] = input_num;
		}

		return ValidationResult::Accept(retun_val);
	}

	ValidationResult::Reject
}

fn main() {
	io::stdout().flush();
	
	let turn_limit = 9;
	let mut turn = 1;

	let answer = generate_answer();

	println!("[DEBUG] answer : {:?}", answer);
	println!(">>> Game started...");

	'main_turn: loop {	
		if turn > turn_limit {
			println!("game over...");
			return;
		}

		let mut input: String = String::new();
		println!("[turn : {}] Input your answer", turn);
		io::stdin().read_line(&mut input);

		// input format validation
		let input_numbers = match convert_input(input) {
			ValidationResult::Accept(numbers) => numbers,
			ValidationResult::Reject => {
				println!("check your input format");
				println!("input format : number number number");
				println!("example : 1 2 3");

				continue 'main_turn;
			}
		};
		let mut current_result = Result{strike:0, ball:0};

		// compare with answer
		for (idx, each_number) in input_numbers.iter().enumerate() {
			for (answer_idx, each_answer) in answer.iter().enumerate() {
				if *each_number == *each_answer {
					if idx == answer_idx {
						current_result.strike += 1;
					} else {
						current_result.ball += 1;
					}
				}
			}
		}

		println!("[turn : {}] strike : {}, ball : {}\n", turn, current_result.strike, current_result.ball);

		if current_result.strike == 3 {
			println!("clear!");
			println!("your score : {} turn", turn);
			return;
		}

		turn += 1;
	}
}