//struct definition
struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

// Using Tuple Structs without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit-Like Structs Without Any Fields
struct AlwaysEqual;


fn main() {
	let mut user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	user1.email = String::from("anotherEMAIL@MAIL.COM");

	let user2 = build_user(String::from("email2@mail.com"), String::from("username2"));

	// sometimes we want to build a struct using a lot of the same values as another instance

	// we can do struct update syntax (it's similar to js object destructuring)

	//Before struct update syntax:
	let user3 = User {
		active: user1.active,
		username: user1.username,
		email: String::from("another@example.com"),
		sign_in_count: user1.sign_in_count,
	};

	//After struct update syntax:
	let user4 = User {
		email: String::from("another@example.com"),
		..user3
	};

	print_user_details(&user2);
	print_user_details(&user4);

	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);

	let subject = AlwaysEqual;
}

fn print_user_details(user: &User) {
	println!("email: {}", user.email);
	println!("username: {}", user.username);
	println!("active: {}", user.active);
	println!("sign in count: {}", user.sign_in_count);
	println!();
}

fn build_user(email: String, username: String) -> User {
	User {
		email: email,
		username: username,
		active: true,
		sign_in_count: 1,
	}
}
