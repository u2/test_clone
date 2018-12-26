#[derive(Clone)]
pub struct A<P> {
    pub s: P
}

#[derive(Copy, Clone)]
pub struct Stats {
   pub frequencies: [i32; 100],
}

#[derive(Clone)]
pub struct Wrapper<T> {
    pub inner: T,
}

pub struct SS<T> {
    pub bar: Wrapper<T>,
}

// #[derive(Clone)]
// struct Wrapper<T> {
//     pub inner: T,
// }
//
// failed
// #[derive(Clone)]
// struct SS<T> {
//     pub bar: Wrapper<T>,
// }

fn main() {
    println!("Hello, world!");
}
