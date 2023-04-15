fn main(){
	let mut n=1;
	while n<100 {
		if n%15 == 0 {
			println!("YN");
		} else if n%3 == 0 {
			println!("Y");
		} else if n%5 == 0 {
			println!("N");
		} else {
			println!("{}",n);
		}
		
		n+=1;
	}
}
