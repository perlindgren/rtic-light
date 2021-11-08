use pass2::pass2;
use rtic::app;

#[app(passes = [pass2, oeuntheuo], plepps = 7)]
mod b {
    // here is some code
    fn a() {}
    fn main() {
        println!("{}", answer());
    }
}
