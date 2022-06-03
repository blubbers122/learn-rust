#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}


impl Rectangle {
	// The &self is actually short for self: &Self
	// the type Self is an alias for the type that the impl block is for
	fn area(&self) -> u32 {
		self.width * self.height
	}

	// we can have a function share the same name as a struct field
	fn width(&self) -> bool {
		self.width > 0
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}

	// this method does not have self as a parameter, so it acts like a 'static' method.
	// we don't need to instantiate a Rectangle to call this method
	// we can call like Rectangle::square(3)
	//This function is namespaced by the struct
	fn square(size: u32) -> Rectangle {
		Rectangle {
			width: size,
			height: size,
		}
	}
}

fn main() {
	let scale = 2;
	let rect1 = Rectangle {
		width: dbg!(30 * scale), // dbg! is a macro that prints the value of the expression it's given to the console
		height: 50,
	};

	//Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug
	println!("rect1 is {:?}", &rect1); // unformatted

	println!("rect1 is {:#?}", &rect1); // formatted

	dbg!(&rect1); // formatted

	if rect1.width() {
		println!("The rectangle has a nonzero width; it is {}", rect1.width);
	}

	println!(
		"The area of the rectangle is {} square pixels.",
		rect1.area()
	);

	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};
	let rect3 = Rectangle {
		width: 60,
		height: 45,
	};

	let square = Rectangle::square(3);

	println!("Can square hold rect1? {}", square.can_hold(&rect1));

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
