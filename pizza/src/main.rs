use std::io;
use std::cmp;
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
    let a = next_num();
    let b = next_num();
    let ab = next_num();
    let no_a = next_num();
    let no_b = next_num();
    let price =  no_a*a + no_b*b;
    static mut RES : usize = 0;
    unsafe{RES = price};
    for i in 1..cmp::max(no_a, no_b)+1 {
	let no_ab = i*2;
	let price : usize = no_ab*ab + no_a*cmp::max(0, no_a-i) +  no_b*cmp::max(0, no_b-i);
	unsafe {RES = cmp::max( price , RES);}
    }
    unsafe{println!("{}" , RES);}
}
