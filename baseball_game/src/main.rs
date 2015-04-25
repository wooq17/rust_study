use std::io;
use std::io::Write;

struct Result {
	strike: u32,
	ball: u32,
}

fn main() {
	io::stdout().flush();
	
	let turn_limit = 9;
	let mut turn = 0;

	let answer = [3,6,8];

	println!(">>> Game started...");

	 'main_turn: loop {		
	 	if turn > turn_limit {
			println!("game over...");
			return;
		}

		let mut input = String::new();
		println!("[turn : {}] Input your answer", turn);
		io::stdin().read_line(&mut input);

		// input validation
		let input_numbers: Vec<&str> = input.trim().split(' ').collect();
		let mut current_result = Result{strike:0, ball:0};

		let element_count = input_numbers.len();
		if element_count > 3 {
			println!("too many numbers...");
			continue;
		}

		for each_number_str in &input_numbers {
			let each_number = i32::from_str_radix(each_number_str, 10);

			let input_num = match each_number {
				Ok(num) => num,
				Err(_) => {
					println!("check your input format");
					println!("input format : number number number");
					println!("example : 1 2 3");
					continue 'main_turn;
				}
			};

			println!("vaild");
		}

		turn += 1;
	}
}