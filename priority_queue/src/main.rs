struct PriorityQueue<T: Ord> {
	elements: Vec<T>,
	size: usize,
}

impl<T> PriorityQueue<T> where T: Ord + Default + Clone {
	pub fn new() -> PriorityQueue<T> {
		PriorityQueue { elements: vec![Default::default()], size: 0 }
	}

	pub fn push(&mut self, element: T) {
		self.size += 1;
		let mut current_idx = self.size;

		if self.size >= self.elements.len() {
			self.elements.push(element.clone());
		} else {
			self.elements[current_idx] = element.clone();
		}

		while current_idx/2 > 0 
				&& self.elements[current_idx] < self.elements[current_idx/2] {
			self.elements.swap(current_idx, current_idx/2);
			current_idx /= 2;
		}
	}

	pub fn pop(&mut self) {
		if self.size > 0 {
			self.elements.swap(1, self.size);
			self.size -= 1;
			self.heapify(1);
		}
	}

	pub fn peek(& self) -> T {
		if self.size > 0 {
			self.elements[1].clone()
		} else {
			println!("no element : default");
			Default::default()
		}
	}

	pub fn size(& self) -> usize {
		self.size
	}

	fn heapify(&mut self, idx: usize) {
		let left = idx*2;
		if left > self.size { return; }

		let mut large_idx = left;
		if left+1 <= self.size && self.elements[large_idx] > self.elements[left+1] { large_idx += 1; }

		if self.elements[idx] > self.elements[large_idx] {
			self.elements.swap(idx, large_idx);
			self.heapify(large_idx);
		}
	}
}

fn main() {
    println!("PriorityQueue test");

    let mut test_queue = PriorityQueue::<i32>::new();

    test_queue.push(3);
    test_queue.push(-1);
    test_queue.push(5);
    test_queue.push(2);
    test_queue.push(1);
    test_queue.push(1);
    test_queue.push(-8);
    test_queue.pop();
    test_queue.push(-7);

    while test_queue.size() > 0 {
    	print!("{} ", test_queue.peek());
    	test_queue.pop();
    }

    print!("\n");
}
