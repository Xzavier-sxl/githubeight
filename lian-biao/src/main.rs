use List::*;

enum List {
	Cons( u32, Box<List>),	//元组结构体，包含链表的一个元素和一个指向下一节点的指针 （ 元素， 指向下一个节点的指针）
	
	Nil,	//末尾节点，指连表结束
}

impl List {

	fn new() -> List {
		Nil
	}
	
	fn prepend(self, elem: u32) -> List {
		
		Cons(elem, Box::new(self))
	} 
	
	fn len(&self) -> u32 {
		
		match *self {
			
			Cons( _ , ref tail ) => 1+tail.len(),
			Nil => 0
		}
	}
	
	
	fn stringify(&self) -> String {
		match *self {
			Cons(head, ref tail) => {
				format!("{},{}",head, tail.stringify())
			},
			
			Nil => {
				
				format!("Nil")
			},
			
		}
	}
}


fn main() {
	
	let mut list = List::new();	//创建一个空链表
	
	list = list.prepend(1);
	list = list.prepend(2);
	list = list.prepend(3);
	
	println!("{}",list.len());
	println!("{}",list.stringify());
}

















