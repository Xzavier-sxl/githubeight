#[derive(Debug)]
struct Student<'a> {
	name: &'a str,
	gender: &'a str,
	height: u32,
}

fn main(){
	let xzavier = Student{
		name: &String::from("Xzavier"),
		gender: &String::from("Male"),
		height: 178,
	};
	
	println!(":#?",xzavier);
}
