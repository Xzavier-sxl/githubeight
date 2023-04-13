fn main(){
	let mut count = 0u32;
	
	loop {
		count += 1;
		
		if count == 3{
			println!("Three");
			
			continue;
		}
		println!("{}",count);
		
		if count == 5 {
			println!("Five");
			
			break;
		}
	}
}
