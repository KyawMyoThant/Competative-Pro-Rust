use std::io;
fn main() {
    //open a reader 
    let mut val = String::new();
    io::stdin()
    .read_line(&mut val)
    .expect("read line error");
    
    //reading a line and split it into two numbers
    let mut substr_iter = val.split_whitespace();
    let mut next_num = || -> usize {
        substr_iter.next().expect("Not enough input numbers")
                   .parse().expect("Input is not a number")
    };
    let val1 = next_num();
    let val2 = next_num();

    //real checking start here
    static mut RES : u32 = 0;
    for a in 0..val1-1 {
	for b in a+1..val1{
	    for c in b+1..val1+1{
		if a+b+c == val2
		{unsafe{RES= RES+1;}}
	    }
	}
    }
    unsafe{println!("{}",RES);}
}
