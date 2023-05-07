// enum IpAddrKind {
// 	V4,
// 	V6,
// }

enum IpAddr {
	// V4(String),
	V4(u8, u8, u8, u8),
	V6(String),
}

enum Message {
	Quit, //Quit has no data associated with it at all.
	Move { x: i32, y: i32 }, //Move has named fields like a struct does.
	Write(String), //Write includes a single String
	ChangeColor(i32, i32, i32), //ChangeColor includes three i32 values.
}



// struct IpAddr {
// 	kind: IpAddrKind,
// 	address: String,
// }

fn main() {
	// let four = IpAddrKind::V4;
	// let six = IpAddrKind::V6;

	// let home = IpAddr {
	// 	kind: IpAddrKind::V4,
	// 	address: String::from("127.0.0.1"),
	// };

	// let loopback = IpAddr {
	// 	kind: IpAddrKind::V6,
	// 	address: String::from("::1"),
	// };

	// let home = IpAddr::V4(String::from("127.0.0.1"));
	let home = IpAddr::V4(127, 0, 0, 1);


	let loopback = IpAddr::V6(String::from("::1"));

}
