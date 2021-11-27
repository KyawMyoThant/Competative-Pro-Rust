use std::io;
fn main() {
    //Take a String input
    let mut st = String::new();
    io::stdin().read_line(&mut st).expect("Failed to read String");

    //Replace the other charactor with whitespace
    let mut new_st = String::new();
    for i in st.chars(){
	if "ACGT".contains(i){
	    new_st.push(i);
	}else{
	    new_st.push(' ');
	}
    }
    let v = new_st.split_whitespace();
    println!("{}" , v.max().unwrap());
}
