fn main() {
	let long_lived_binding = 1;
	
	{
		let short_lived_binding = 2;
		println!("{}",short_lived_binding);
		
		let long_lived_binding = 5_f32;
		println!("{}",long_lived_binding);
	}
	
	println!("{}",long_lived_binding);
	
	let long_lived_binding = 'a';
	println!("{}",long_lived_binding);
}
