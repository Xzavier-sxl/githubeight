use std::fmt;

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);  //元组

impl Matrix {
	fn transpose(&self) -> Matrix{
		Matrix(self.0,self.2,self.1,self.3)
	}	
}

impl fmt::Display for Matrix {
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"({} {})\n",self.0,self.1)?;
		write!(f,"({} {})",self.2,self.3)
	}
}


fn main(){
	let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
	println!("{}",matrix);
	println!("Matrix:\n{}",matrix);
	println!("Transpose:\n{}",matrix.transpose());
}
