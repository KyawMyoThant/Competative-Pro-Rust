use std::io;
use std::cmp;
fn main() {
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("read line error");
    
    //reading a line and split it into two numbers
    let mut substr_iter = val.split_whitespace();
    let mut next_num = || -> usize {
        substr_iter.next().expect("Not enough input numbers")
                   .parse().expect("Input is not a number")
    };
    let val1 = next_num();
    let val2 = next_num();
    
    let mut point_vec = vec![vec![0; val2]; val1];
    for i in 0..val1{
    let mut point = String::new();
    io::stdin().read_line(&mut point).expect("read line error");
    
    let mut subpoint_itr = point.split_whitespace();
    let mut next_point = || -> usize {
        subpoint_itr.next().expect("Not enough input numbers")
                   .parse().expect("Input is not a number")
    };

	for j in 0..val2{
	    point_vec[i][j] = next_point();
	}
    }
    let mut res = 0;
    for i in 0..val2 {
	for j in 0..val2{
	    let mut sum = 0;
	    for k in 0..val1{
		sum += cmp::max(point_vec[k][i], point_vec[k][j]); 
	    }
	    res = cmp::max(res, sum);
	}
    }
    println!("{}", res);
}
