use pass1::*;
use pass2::*;
use rtic::app;

#[app(passes = [pass2], plepps = 7)]
mod b {
    // here is some code
    fn a() {}
    fn main() {
        println!("{}", answer());
    }
}
