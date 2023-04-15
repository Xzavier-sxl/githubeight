fn main(){
	let number = 13;
	println!("{}",number);
	
	match number {
		1 => println!("one"),
		
		2|3|5|7|11 => println!("prime"),
		
		13..=19 => println!("teen"),
		
		_ => println!("special"),
	}
	
	let boolean = true;
	
	let binary = match boolean {
		false => 0,
		true => 1,
	};
	
	println!("{} -> {}",boolean,binary);
}
