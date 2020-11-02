pub struct Basket {
    capacity: u64,
    empty: bool,
    full: bool,
    content: u64
}

impl Basket {
	pub fn is_full(&self) -> bool {
		return self.full;
	}
}

impl Basket {
	pub fn is_empty(&self) -> bool {
		return self.empty;
	}
}

impl Basket {
	fn update(&mut self) {
		self.empty = false;
		self.full = false;
		if self.content == self.capacity {
			self.full = true;
		}
		else if self.content == 0 {
			self.empty = true;
		}
	}
}

impl Basket {
	pub fn add(&mut self, n: u64) {
		self.content += n;
		if self.content >= self.capacity {
			self.content = self.capacity;
		}
		self.update();
	}
}

impl Basket {
	pub fn remove(&mut self, n: u64) {
		if n >= self.content {
			self.content = 0;
		}
		else {
			self.content -= n;
		}
		self.update();
	}
}


#[test]
fn empty_basket() {
	let mut b = Basket {
		capacity: 10,
		content: 0,
		full: false,
		empty: false
	};
	b.remove(5);
    assert_eq!(b.is_full(), false);
    assert_eq!(b.is_empty(), true);
}

#[test]
fn non_empty_basket() {
	let mut b = Basket {
		capacity: 10,
		content: 0,
		full: false,
		empty: false
	};
	b.add(5);
    assert_eq!(b.is_full(), false);
    assert_eq!(b.is_empty(), false);
}

#[test]
fn full_basket() {
	let mut b = Basket {
		capacity: 10,
		content: 0,
		full: false,
		empty: false
	};
	b.add(10);
    assert_eq!(b.is_full(), true);
    assert_eq!(b.is_empty(), false);
}

#[test]
fn roundtrip_basket() {
	let mut b = Basket {
		capacity: 10,
		content: 0,
		full: false,
		empty: false
	};
	b.add(20);
    assert_eq!(b.is_full(), true);
    assert_eq!(b.is_empty(), false);
	b.remove(1);
    assert_eq!(b.is_full(), false);
    assert_eq!(b.is_empty(), false);
	b.remove(9);
    assert_eq!(b.is_full(), false);
    assert_eq!(b.is_empty(), true);
}