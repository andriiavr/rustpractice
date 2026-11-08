/*
let my_num: i32 = 5;          integer
let my_double: f64 = 5.99;    float
let my_letter: char = 'D';    character
let my_bool: bool = true;     boolean
let my_text: &str = "Hello";  string
*/

// use std::collections::HashMap;

use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();
		
  	let mut total = 1;
	let mut num = n;
    while num != 0 {
		println!("{} {}", total, num);
		let lastn = num % 10;
		num /= 10;
		total *= lastn;
		println!("{} {} {}", total, num, lastn);
	};
	println!("{}", total)
}

