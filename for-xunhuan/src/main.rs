fn main(){
	//let names = vec!["Bob","Frank","Ferris"];
	
	/*for name in names.iter() {
		match name {
			&"Ferris" => println!("Xzavier"),
			_ => println!("Hello {}",name),
		}
	}*/
	
	
	/*for name in names.into_iter() {
		match name {
			"Ferris" => println!("Xzavier"),
			_ => println!("Hello {}",name),
		}
	}*/
	
	let mut names = vec!["Bob","Frank","Ferris"];
	
	for name in names.iter_mut() {
		*name = match name {
			&mut "Ferris" => "Xzavier",
			_ => "Hello",
		}
	}
	println!("{:?}",names);
}
