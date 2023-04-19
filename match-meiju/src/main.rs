#[allow(dead_code)]	//消除警告

enum Color {
	Red,
	Blue,
	Green,
	
	RGB(u32,u32,u32),
	HSV(u32,u32,u32),
	HSL(u32,u32,u32),
	CMY(u32,u32,u32),
	CMYK(u32,u32,u32,u32),
}

fn main() {
	let color = Color::RGB(122,17,40);
	
	println!("color?");
	
	match color {
		Color::Red => println!("Red"),
		Color::Blue => println!("Blue"),
		Color::Green => println!("Green"),
		Color::RGB(r,g,b) =>
			println!("{} {} {}",r,g,b),
		Color::HSV(h,s,v) =>
			println!("{} {} {}",h,s,v),
		Color::HSL(h,s,l) =>
			println!("{} {} {}",h,s,l),
		Color::CMY(c,m,y) =>
			println!("{} {} {}",c,m,y),
		Color::CMYK(c,m,y,k) =>
			println!("{} {} {} {}",c,m,y,k),
	}
}
