extern crate fizz_rust;

#[allow(dead_code)]
fn main() {
    for i in (1..101).map(fizz_rust::to_fizz_rust) {
        println!("{:?}", i);
    }
}
