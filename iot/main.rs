
fn main(){

use std::io;
	println!("Kindly fill up the required information to get your ID");
	//taking first user input
	let  mut a=String::new();
	println!("Enter your name");
	io::stdin().read_line(&mut a).expect("Incorrect entry");
	a=a.trim().to_string();
	//println!("Name={}\nDate of birth:{}\n",a);
	//2nd user  input
	let  mut b=String::new();
	println!("Enter Your date of birth");
	io::stdin().read_line(&mut b).expect("Incorrect entry");
	b=b.trim().to_string();
	//3rd user input
	let  mut c=String::new();
	println!("Enter your roll number");
	io::stdin().read_line(&mut c).expect("Incorrect entry");
	c=c.trim().to_string();
	//4th user input
	let  mut d=String::new();
	println!("Enter your City name");
	io::stdin().read_line(&mut d).expect("Incorrect entry");
	d=d.trim().to_string();
	//5th user input
	let  mut e=String::new();
	println!("Enter your batch timings");
	io::stdin().read_line(&mut e).expect("Incorrect entry");
	e=e.trim().to_string();
	
	println!("Name={}                ",a);
	println!("Date of Birth:{}",b);
	println!("Roll num:{}",c);
	println!("City:{}",d);
	println!("timings:{}",e);


















}
	
