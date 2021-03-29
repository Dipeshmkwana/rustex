mod world_file;
mod hello_file;
pub use crate::world_file::world_mod::hello;

fn main() {
    println!("Hello, world!");
    hello();
}
