mod integers;
mod cyclotomic;

use integers::euler_phi;
use cyclotomic::CyclotomicInteger;

fn main() {
    println!("Hello, world!");

    println!("{}", euler_phi(420))

}
