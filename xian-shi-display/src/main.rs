use std::fmt;

#[derive(Debug)]
struct Point2D {
	x: f64,
	y: f64,
}

impl fmt::Display for Point2D {
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"{}+{}i",self.x,self.y)
	}
}

fn main(){
	let point = Point2D{
		x: 3.3,
		y: 7.2,
	};
	
	println!("{}",point);
	println!("{:#?}",point);
}
