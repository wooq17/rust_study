trait Sortable {
	fn quick_sort(&mut self);
}

impl Sortable for [i32] {
	fn quick_sort(&mut self) {
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

		
		if post_part_start_idx > 1 {
			let mut prev_part = &mut self[0..post_part_start_idx-1];
			prev_part.quick_sort();
		}
		
		if  pivot_idx > 0 && post_part_start_idx < pivot_idx {
			let mut post_part = &mut self[post_part_start_idx..pivot_idx+1];
			post_part.quick_sort();
		}
	}
}

fn main() {
    // let mut test_vector:Vec<i32> = vec![4, 6, 9, 3, -1, -5, 7, 3, 6, -1, 2];
    let mut test_vector = [4, 6, 9, 3, -1, -5, 7, 3, 6, -1, 2];
    println!("before : {:?}", test_vector);

    test_vector.quick_sort();

    println!("after  : {:?}", test_vector);
}

