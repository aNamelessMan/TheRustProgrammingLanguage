/*
$ rustc --version
$ rustup update
$ rustup doc
$ rustfmt ./file

$ cargo --version
$ cargo new hello_cargo
$ cd hello_cargo

$ cargo build
default build is a debug build
$ cargo build --release

$ cargo run
can be used to compile and run!!! 
but Cargo do NOT watches for file changes 实测修改代码也会失效，应该是教程有问题


$ cargo check

$ git clone example.org/someproject
$ cd someproject
$ cargo build
*/
fn main() {
    println!("Hello, world!");
}
