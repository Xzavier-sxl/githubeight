fn main() {
	let a_binding;	//声明一个变量绑定
	
	{
		let x = 2;
		a_binding = x*x;
	}
	
	println!("{}",a_binding);
	
	let another_binding;
	
	another_binding = 1;
	println!("{}",another_binding);
}
