#![allow(overflowing_literals)]

fn main() {
	let decimal = 65.4321_f32;
	
	let integer = decimal as u8;
	let character = integer as char;
	
	println!("{} -> {} -> {}",decimal,integer,character);
	
	println!("{}",1000 as u16);
	println!("{}",1000 as u8);
	
	println!("{}",(-1i8) as u8);
	
	println!("{}",1000 % 256);
	
	println!("{}",128 as i16);
	println!("{}",128 as i8);
	
	println!("{}",1000 as u8);
	println!("{}",232 as i8);
}
