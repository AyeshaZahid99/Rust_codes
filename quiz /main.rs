extern crate colorful;
use std::process;
use colorful::Color;
use colorful::Colorful;
use std::thread;
use std::time::Duration;

fn main(){ 
	let output="WELCOME TO ENTRY TEST 2019";
	println!("{}",output.color(Color::Yellow).bold());




	use std::io;
	let mut input=String::new();
	println!("Please enter your percentage");
	io::stdin().read_line(&mut input).expect("incorrect entry");
	let input:f32=input.trim().parse().unwrap();
	loop {
	if input >75.0{
	println!("You are eligible for the test");
	let comp="Good luck!!";
	println!("{}",comp.color(Color::Pink3).bold());

thread::spawn(|| {
	println!("you have 20 seconds to complete the quiz!!");
	thread::sleep(Duration::from_secs(20));
	println!("your time is up!");
	process::exit(0);
});


	}
	else {
	println!("We are sorry you can't apply");
	break
	}



	let mut answer_01=String::new();
	println!("Q:1\nAre variables mutable in RUST?\na:yes 	b:no");
	io::stdin().read_line(& mut answer_01).expect("incorrect entry!");
	answer_01=answer_01.trim().to_string();
	// time limit
	thread::spawn(||{
	thread::sleep(Duration::from_secs(5));
	process::exit(0);
});

	let options=["a","b","c","d"];
	let mut points=0;
	if answer_01==options[0]{
	let cp="Correct Answer!!";
	println!("{}",cp.color(Color::Green).bold());
	points+=1
	}
	else{
	let wa="Wrong Answer!!";
	println!("{}",wa.color(Color::Red));
	}
	
	

	let mut answer_02=String::new();
	println!("Q:2\nThe default integer type in RUST is?\na:i32 b:u8");
	io::stdin().read_line(& mut answer_02).expect("incorrect answer");
	answer_02=answer_02.trim().to_string();
	let op=["a","b","c","d"];
	thread::spawn(||{
	thread::sleep(Duration::from_secs(5));
	process::exit(0);
});



	if answer_02==op[0]{
	let co="Correct Answer!!";
	println!("{}",co.color(Color::Green).bold());
	points+=1	
	}
	else {
	let wans="Wrong answer!!";
	println!("{}",wans.color(Color::Red));
}

	let mut answer_03=String::new();
	println!("Q:3\nWhich one of them is an unsigned integer?\na:i32  b:u32");
	io::stdin().read_line(&mut answer_03).expect("Incorrect entry!");
	answer_03=answer_03.trim().to_string();
	let ot=["a","b"];
	if answer_03==ot[1] {
	let ca="Correct Answer!!";
	println!("{}",ca.color(Color::Green).bold());
	points+=1
	}
	else {
	let wr="Wrong answer!!";
	println!("{}",wr.color(Color::Red));
}

	println!(" Your score is {}",points);

	if points==3 {
	let result="Congratulations you have passed the Entry Test!!";
	println!("{}",result.color(Color::Yellow).bold());
	break
}
 	else {
	let fail="You are fail";
	println!("{}",fail.color(Color::Yellow).bold());
	break
}
	}
}
