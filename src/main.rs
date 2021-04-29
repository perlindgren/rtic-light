use pass1::*;
use pass2::*;
use rtic::app;

#[app(passes = [pass1, pass2], plepps = 7)]
mod b {}

fn main() {
    println!("{}", answer());
}
