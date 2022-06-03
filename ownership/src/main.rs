fn main() {
	// s is not valid here, it’s not yet declared

	let mut s2 = String::from("hi");
	s2.push_str(" there!");

	println!("{}", s2);

	let str1 = String::from("hello");

	// this looks like we set str2 to a copy of str1, but str2 creates new data on the stack
	// and reuses str1's pointer to the string allocated on the heap when str1 was initialized
	let str2 = str1;

	// rust invalidates str1 after str2 is assigned to str1 to prevent a double free error and promote memory safety and security
	// this line below will throw a compiler error bc rust disables str1
	// println!("{}, world!", str1);

	let x = 5;
	let y = x;

	// rust doesn't invalidate the variable x here because integers (and all scalar values) are stored entirely on the stack
	// hence no heap data with double pointers to worry about
	println!("x = {}, y = {}", x, y);

	let s = String::from("hello"); // s comes into scope

	// s's value moves into the function...
	// ... and so is no longer valid here
	takes_ownership(s);
	// below line of code would fail because the above function takes ownership and s becomes unusable in any point
	// in the current scope after takes_ownership()'s invocation, because it was moved into another scope
	// println!("{}", s);

	let x = 5; // x comes into scope

	// x would move into the function,
	// but i32 is Copy, so it's okay to still
	// use x afterward
	makes_copy(x);

	let str_given_to_me = gives_ownership(); // gives ownership gives us the ownership of a value initialized in it's scope

	let a = takes_and_gives_back(str_given_to_me); // we temporarily give ownership of str_given_to_me to takes_and_gives_back and assign the value returned to a

	println!("{}", a);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
	// special happens.

fn gives_ownership() -> String {
	// gives_ownership will move its
	// return value into the function
	// that calls it

	let some_string = String::from("yours"); // some_string comes into scope

	some_string // some_string is returned and
	          // moves out to the calling
	          // function
}

fn takes_and_gives_back(a_string: String) -> String {
	// a_string comes into
	// scope

	a_string // a_string is returned and moves out to the calling function
}

fn takes_ownership(some_string: String) {
	// some_string comes into scope
	println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
	// memory is freed.

fn makes_copy(some_integer: i32) {
	// some_integer comes into scope
	println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
