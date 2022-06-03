//SUMMARY

// At any given time, you can have either one mutable reference or any number of immutable references.

// References must always be valid.


fn main() {
	let s1 = String::from("hello");

	// &s1 is a reference to the value stored in variable 's1'
	// references are a good way to allow a variable to be used again (without being forced to return it again)
	// after being passed as a param to a function call

	let len = calculate_length(&s1);

	// s1 is still available after the above function call because we passed a reference to s1 instead of the variable itself
	println!("The length of '{}' is {}.", s1, len);

	let mut s = String::from("hello");

	change(&mut s);

	// Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time.
	// this limitation prevents data races at compile time

	// one way to have 2 references is to create a new scope like this example below vvv
	{
		let r1 = &mut s;
		change(r1);
	} // r1 goes out of scope here, so we can make a new reference with no problems.

	let r2 = &mut s;
	change(r2);


	// rust doesn't like creating immutable and mutable references at the same time,
	// but since we know mutable references, r1 and r2 have been 'disposed of' from the change function calls, we can
	// safely create an immutable reference as we do below
	let r3 = &s;


	dangling_references();


	println!("S was changed to: {}", s);
}

fn dangling_references() {
	// In Rust the compiler guarantees that references will never be dangling references:
	// if you have a reference to some data, the compiler will ensure that the data will
	// not go out of scope before the reference to the data does.

	// let reference_to_nothing = dangle();
	let str = no_dangle();

}

fn no_dangle() -> String {
	let s = String::from("hello");
	s
}

// fn dangle() -> &String {
// 	let s = String::from("hello");

// 	&s
// } // we try to return a reference to the string 's', but at the end of this function, s goes out of scope,
// making the pointer point to nothing and 'dangle'. Rust doesn't like this and throws a compiler error

// we can't change a value (it's immutable) when only given a reference
// vvv this code won't compile: vvv
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// this function works because we pass it the mutable string reference type
fn change(some_string: &mut String) {
	some_string.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
	// s is a reference to a String
	s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
	// it refers to, nothing happens (no dropping value).
