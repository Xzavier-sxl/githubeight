enum Foo {Bar}

fn main() {
	let a = Foo::Bar;
	
	if let Foo::Bar = a {		//处理仅有一次匹配的情况
		println!("foobar");
	}
}
