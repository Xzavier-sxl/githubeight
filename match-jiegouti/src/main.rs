fn main(){
	struct Foo {
		x: (u32,u32),
		y: u32,
	}
	
	let foo = Foo {x: (1,2),y: 3};
	
	let Foo {x: (a,b),y} = foo;
	
	println!("{} {} {}",a,b,y);
	
	let Foo {x: i,y: j} = foo;
	
	println!("{:?} {:?}",i,j);
	
	let Foo {y,..} = foo;
	
	println!("{}",y);
}
