use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
pub struct Foo<T>(Arc<T>);

#[derive(Clone)]
pub struct Shared<C, B> {
    store: Arc<C>,
    bar: B,
    l: Arc<RwLock<C>>
}

fn main() {
    println!("Hello, world!");
}
