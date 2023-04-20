fn age() -> u32 {
	15
}

fn main() {
	match age() {
		0	=> println!("zero"),
		n @ 1  ..= 12 => println!("child of age {:?}",n),
		n @ 13 ..= 19 => println!("teen of age {:?}",n),
		n	=> println!("old person of age {:?}",n),
	}
}
