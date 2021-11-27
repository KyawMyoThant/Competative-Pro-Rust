use std::io;
fn main() {
    // take user input
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed to read input");
    let y = x.trim().parse::<i32>().unwrap();
    static mut RES: i32 = 1;


    fn divisor(u: i32) -> i32 {
	let mut counter = 0;
	for i in 1..u+1 {
	    if u % i == 0
	    {
		counter = counter + 1;
	    }

	}
	//println!("{}" , counter);
	counter
    }

    //checking if under or equal to 105
    if y < 105 {
	println!("{} " , 0);
    } else if y == 105 {
	println!("{}", 1);
    } else {
	for i in 106..y {
	    if  i%2 == 1{
		if divisor(i) == 8
		{
		   // println!("{}" , i);
		    unsafe{RES = RES + 1;}; }
	    }
	}
	
	unsafe{println!("{}" , RES);}
    }
}
