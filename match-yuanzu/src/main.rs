fn main(){
	let triple = (0,-2,3);
	println!("{:?}",triple);
	
	match triple {
		(0,x,y) => println!("{:?}",triple),
		(1, ..) => println!("matter"),  //忽略其余元组部分
		_	=> println!("It donesn't matter what they are"),
	}
}
