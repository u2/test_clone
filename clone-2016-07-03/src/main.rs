#[derive(Clone)]
pub struct A<P> {
    pub s: P
}

#[derive(Copy, Clone)]
pub struct Stats {
   pub frequencies: [i32; 100],
}

fn main() {
    println!("Hello, world!");
}
