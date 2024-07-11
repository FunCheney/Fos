#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

const WIDITH: usize = 10;
const HEIGHT: usize = 5;

#[no_mangle]
fn main() -> i32 {
    for i in 0..HEIGHT  {
       for _ in 0..WIDITH  {
           print!("A");
       } 

       println!("[{}/{}]", i+1, HEIGHT);
    }
    println!("[user] Test write_a OK");
    0
}

