#![allow(dead_code)]

enum WebEvent {
	PageLoad,
	PageUnload,
	
	KeyPress(char),//元组结构体
	Paste(String),
	
	Click { x: i64, y: i64 }  //普通结构体
}
fn inspect (event: WebEvent) {
	match event {
		WebEvent::PageLoad => println!("page loaded"),
		WebEvent::PageUnload => println!("page unloaded"),
		
		WebEvent::KeyPress(c) => println!(" '{}' ",c),
		WebEvent::Paste(s) => println!(" {} ",s),
		
		WebEvent::Click { x, y} => {
			println!(" x={}, y={} ",x,y);
		},
	}
}

fn main() {
	let pressed = WebEvent::KeyPress('x');
	let pasted = WebEvent::Paste("my test".to_owned());
	
	let click = WebEvent::Click{x:20,y:80};
	
	let load  = WebEvent::PageLoad;
	let unload = WebEvent::PageUnload;
	
	inspect(pressed);
	inspect(pasted);
	inspect(click);
	inspect(load);
	inspect(unload);
}

