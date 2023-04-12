#[derive(Debug)]
struct Number {
	value: u32,
}

impl From<u32> for Number{
	fn from(item: u32) -> Self{
		Number{	value: item }
	}
}

fn main(){
	let my_str = "Hello";
	let my_string = String::from(my_str);	//字符串转变String
	
	let num = Number::from(50);
	println!("{:#?}",num);
	
	let int =5;
	let num: Number = int.into();
	println!("{:#?}",num);
	
}
