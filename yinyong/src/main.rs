fn main(){	
	let reference = &4;	
	match reference {		
		&val => println!("{:?}",val),	
	}		
	match *reference {		
		val => println!("{:?}",val),	
	}		
	
	
	let _not_a_reference = 3; 		
	
	let ref _is_a_reference = 3;		
	
	let value = 5;	
	let mut mut_value = 6;	
	match value {		
		ref n => println!("{:?}",n),	
	}		
	match mut_value {		
		ref mut m => {			
			*m += 10;			
			println!("{:?}",m);		
		}	
	}
}
