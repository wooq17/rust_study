trait Sortable {
	fn quick_sort(&mut self);

	fn partition(&mut self) -> usize;
}

impl Sortable for [i32] {
	fn quick_sort(&mut self) {
		let length = self.len();
		if length < 2 { return; }

		let pivot_idx = self.partition();

		self[0..pivot_idx].quick_sort();
		self[pivot_idx+1..length].quick_sort();
	}

	fn partition(&mut self) -> usize {
		let pivot_idx = self.len() - 1;
		let pivot_value = self[pivot_idx];
		let mut post_part_start_idx = 0;

		for current_idx in 0..pivot_idx+1 {
			if self[current_idx] > pivot_value { continue; }

			let temp = self[post_part_start_idx];
			self[post_part_start_idx] = self[current_idx];
			self[current_idx] = temp;

			post_part_start_idx += 1;
		}

		post_part_start_idx-1
	}
}

fn main() {
    // let mut test_vector:Vec<i32> = vec![4, 6, 9, 3, -1, -5, 7, 3, 6, -1, 2];
    let mut test_vector = [4, 6, 9, 3, -1, -5, 7, 3, 6, -1, 2];
    println!("before : {:?}", test_vector);

    // test_vector.sort();
    test_vector.quick_sort();

    println!("after  : {:?}", test_vector);
}

