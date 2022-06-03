fn main() {
	let mut x = 5;
	println!("The value of x is: {}", x);
	x = 6;
	println!("The value of x is: {}", x);

	let myfloat: f32 = 3.5;

	println!("myfloat is: {}", myfloat);

	let heart_eyed_cat = '😻';

	let bear_emoji = '🐻';

	println!("cat: {}, bear: {}", heart_eyed_cat, bear_emoji);

	let tup: (i32, f64, u8) = (500, 6.4, 1);

	let (_, middle, _) = tup;

	println!("middle is {}", middle);

	//index tuple with dot syntax

	let first = tup.0;

	let arr: [i32; 6] = [0, 1, 2, 3, 4, 5];

	// init array of size 5 filled with 3s
	let array_of_threes = [3; 5];

	println!("array of threes last element: {}", array_of_threes[4]);

	//index arr with brackets

	println!("third element in array: {}", arr[2]);

	let months = [
		"January",
		"February",
		"March",
		"April",
		"May",
		"June",
		"July",
		"August",
		"September",
		"October",
		"November",
		"December",
	];
}
