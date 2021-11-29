use std::io;
fn main() {
    //open a reader 
    let mut lucky_pin = String::new();
    io::stdin()
    .read_line(&mut lucky_pin)
    .expect("read line error");
    static mut RES : Vec<String>  = vec![];
    let no_lucky = lucky_pin.len();
    for a in 1..no_lucky-2 {
	for b in a+1..no_lucky-1{
	    for c in b+1..no_lucky{
		let mut pin = String::new();
		pin.push(lucky_pin.chars().nth(a-1).unwrap());
		pin.push(lucky_pin.chars().nth(b-1).unwrap());
		pin.push(lucky_pin.chars().nth(c-1).unwrap());
		unsafe {
		    if !RES.contains(&pin){
			RES.push(pin);
		    }
		}
	    }
	} 
    }
    unsafe{println!("{}", RES.len());}
}
