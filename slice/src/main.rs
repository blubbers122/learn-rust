fn main() {
	let mut s = String::from("hello world");

	// hello and world are slices of the str s
	let hello = &s[0..5];
	let world = &s[6..11];

	let first_two_letters = &s[..2];

	println!("hello: {}", hello);
	println!("world: {}", world);
	println!("first two letters: {}", first_two_letters);

	let first = first_word(&s);

	// this line below will not work because slices (basically a pointer to a section of a string value in memory) are dependent on the original string being in memory
	// s.clear();

	println!("first word: {}", first);

	// string literals are actually immutable slices
	let s2 = "Hello world!";

	// slices can be created from arrays as well
	let a = [1, 2, 3, 4, 5];

	let slice = &a[1..3];

	assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i];
		}
	}

	&s[..]
}
