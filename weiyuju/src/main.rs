fn main(){
	let pair = (2,-2);
	
	println!("{:?}",pair);
	
	match pair {
		(x,y) if x==y => println!("="),	//卫语句
		(x,y) if x+y == 0 => println!("0"),
		(x,_) if x%2 ==1 => println!("odd"),
			_ => println!("correlation.."),
	}
}
