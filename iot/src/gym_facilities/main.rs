 extern crate colorful;
use colorful::Color;
use colorful::Colorful;
fn main(){
	let dis="Welcome to the Slimming Centre 🌟 ";
	println!("{}",dis.color(Color::Yellow));
	use std::io;
	let mut pack=String::new();
	println!("Our available packages are:\n1:yoga   2:zoomba  3: aerobics(advance) 4:classic");
	io::stdin().read_line(&mut pack).expect("Incorrect entry");
	let pack:u8=pack.trim().parse().unwrap();
	if pack==1{
	let pr="500/-";
	println!("The exclusive fascinating package of YOGA will cost you {} per month",pr.color(Color::Yellow));
	let mut res=String::new();
	let c="a:YES    b:NO";
	println!("Are you ready to have this awesome package?\n{}",c.color(Color::Yellow));
	io::stdin().read_line(&mut res).expect("Incorrect entry");
	res=res.trim().to_string();
	let opt=["a","b"];
	if res==opt[0]{
	let co="We welcome you to the gym Kindly pay the fee and join";
	println!("{}",co.color(Color::Yellow));
	let mut res_2=String::new();
	let col="1: 8:00-9:00     2: 12:00-1:00   3:6:00-7:00";
	println!("We have following class timings\n {}",col.color(Color::Yellow));
	io::stdin().read_line(&mut res_2).expect("Incorrect entry");
	let res_2:u8=res_2.trim().parse().unwrap();
	if res_2==1{
	let olo="you are sucessfully enrolled in class of";
	println!("{} 8:00-9:00\n 😊",olo.color(Color::Cyan));}
	else if res_2==2{
	let lm="you are sucessfully enrolled in class of";
	println!("{} 12:00-1:00",lm.color(Color::Yellow));}
	else if res_2==3{
	let mn="you are sucessfully enrolled in class of";
	println!("{} 6:00-7:00",mn.color(Color::Yellow));}
	else{
	let on="No such timings are possible! ⭐️ ";
	println!("{}",on.color(Color::Red));}
}
}


	else if pack==2{
	let pri="2000/-";
	println!("This smartest weight loosing package 'll be offered to you in {} per month",pri.color(Color::Yellow));
	let mut ser=String::new();
	let d="a:YES    b:NO";
	println!("Are you ready to have this awesome package?\n{}",d.color(Color::Yellow));
	io::stdin().read_line(&mut ser).expect("Incorrect entry");
	ser=ser.trim().to_string();
	let er=["a","b"];
	if ser==er[0]{
	let co="We welcome you to the gym,kindly pay the fee!";
	println!("{}",co.color(Color::Yellow));
	let mut ser4=String::new();
	let ol="1: 8:00-9:00     2: 12:00-1:00   3:6:00-7:00";
	println!("We have following class timings\n {}",ol.color(Color::Yellow));
	io::stdin().read_line(&mut ser4).expect("incorrect entry");
	let ser4:u8=ser4.trim().parse().unwrap();
	if ser4==1{
	let l="you are sucessfully enrolled in class of";
	println!("{} 8:00-9:00\n 😊",l.color(Color::Cyan));}
	else if ser4==2{
	let m="U r sucessfully enrolled in class of";
	println!("{} 12:00-1:00",m.color(Color::Yellow));}
	else if ser4==3{
	let p="you are sucessfully enrolled in class of";
	println!("{} 6:00-7:00",p.color(Color::Yellow));}
	else{
	let n="No such timings are possible! ⭐️ ";
	println!("{}",n.color(Color::Red));}
}
}

	else if pack==3{
	let pric="5000/-";
	println!("This quick weight loss and body shaping package 'll cost you {} per month",pric.color(Color::Yellow));}
	else if pack==4{
	let price="1000/-";
	println!("This soothing package 'll cost {} per month",price.color(Color::Yellow));}
	else{
	let no="No such package is available";
	println!("{}",no.color(Color::Yellow));}
	println!("Thanks for coming!\n  🥀");

}
