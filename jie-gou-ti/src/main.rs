#[derive(Debug)]
struct Person{
	name: String,
	age: u8,
}

struct Unit; //单元结构体
#[derive(Debug)]
struct Pair(i32,f32); //元组结构体

struct Point {  //带有字段的结构体
	x: f32,
	y: f32,
}

#[allow(dead_code)]
struct Rectangle {
	top_left: Point,
	bottom_right: Point,
}

impl Rectangle {
	fn rect_area(&self) -> f32 {
		let Rectangle {
			top_left: Point {x: x1, y: y1},
			bottom_right: Point {x: x2, y: y2},
		} = self;
		(x1 - x2).abs() * (y1 - y2).abs()
	}
	
	fn rect_get(tlpoint: Point, length:f32) -> Rectangle {
		Rectangle {
			bottom_right: Point {
				x: tlpoint.x + length,
				y: tlpoint.y - length,
			},
			top_left: tlpoint,
		}
	}
}



fn main(){
	let xzavier = Person {	name: String::from("Xzavier")	,	age: 21 };
	println!("{:?}",xzavier);

	let _unit = Unit;	//实例化一个单元结构体
	
	let pair = Pair(1, 0.1);
	println!("{} {}",pair.0,pair.1);
	println!("{:?}",pair);
	
	let Pair(integer, decimal) = pair;	//解构一个元组结构体
	println!("{:?} {:?}",integer, decimal);
	
	let point: Point = Point{
		x: 10.3,
		y: 0.4,
	};
	println!("({},{})",point.x,point.y);
	
	let bottom_right = Point {	x:5.2,	..point};
	
	println!("({},{})",bottom_right.x,bottom_right.y);
	
	
	let rectangle_ex = Rectangle {
		top_left: Point {x: 10.0,y: 5.0},
		bottom_right: Point {x: 12.0,y: 3.0},
	};
	
	println!("{}",rectangle_ex.rect_area());
	
	let point = Point {x: 5.0,y:6.0};
	let new_rect = Rectangle::rect_get(point, 4.0);
	println!("{}",new_rect.rect_area());
	
}
