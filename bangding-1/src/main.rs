fn some_number() -> Option<u32> {
	Some(42)
}

fn main(){
	match some_number() {
		Some(n @ 42) => println!("answer: {}",n),	//匹配42
		
		Some(n)	     => println!("none: {}",n),		//匹配其他数字
		_            => (),
	}
}
