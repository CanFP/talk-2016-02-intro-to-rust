#![feature(box_syntax)]
fn add_n(n : i32) -> Box<Fn(i32) -> i32 + 'static>  {
    box move |f| n + f
}
 
fn main() {
    let adder = add_n(40);
    println!("The answer to life is {}.", adder(2));
}
