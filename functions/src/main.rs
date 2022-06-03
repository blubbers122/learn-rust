fn main() {
	println!("Hello, world!");

	another_function();

	println!("The sum of 5 and 7 is: {}", sum(5, 7));

	// y is assigned to the result of this block (4)
	let y = {
		let x = 3;
		// not there is no semicolon here, which acts as an implicit return of the result of 3 + 1
		x + 1
	};

	println!("The value of y is: {}", y);

	println!("7 is greater than 5: {}", bigger_than_five(7));

	//ternary like statement
	let num = if 3 > 2 { 1 } else { 2 };

	println!("num: {}", num);

	let arr = [123, 432, 253, 923, 1093];

	print_array_items(arr);
	print_array_items_for(arr);

	let mut counter = 0;

	// we can assign a variable with the result of a loop
	let result = loop {
		counter += 1;

		if counter == 10 {
			// we 'return' counter * 2 in this break statement
			break counter * 2;
		}
	};

	println!("result: {}", result);

	print_1_to_10();
}

fn print_1_to_10() {
	// second num in range here is exclusive.. we don't print out 11 here
	for num in 1..11 {
		println!("num: {}", num);
	}
}

fn print_array_items(arr: [i32; 5]) {
	let mut index = 0;
	while index < 5 {
		println!("index: {}, value: {}", index, arr[index]);
		index += 1;
	}
}

fn print_array_items_for(arr: [i32; 5]) {
	for element in arr {
		println!("value: {}", element);
	}
}

fn another_function() {
	println!("Another function.");
}

fn bigger_than_five(num: i32) -> bool {
	num > 5
}

fn sum(a: i32, b: i32) -> i32 {
	// no need for return keyword.. we have an implicit return with the missing semicolon
	a + b
}
