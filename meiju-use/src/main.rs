#![allow(dead_code)]

enum Status {
	Rich,
	Poor,
}

enum Work {
	Civilian,
	Soldier,
}

fn main() {
	use Status::{ Rich, Poor};  //显式地使用
	
	use Work::*;  //自动地使用’Work‘内部的各个名称
	
	let status = Poor;
	let work = Civilian;
	match status {
		Status::Rich => println!("The rich have losts of money!"),
		Status::Poor => println!("The poor have no money!"),
	}
	
	match work {
		Civilian => println!("Civilian work!"),
		Soldier => println!("Soldier work!"),
	}
}
