mod store;
use std::io;
use store::{Node, write_file};

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    println!("enter data");
    let _ = stdin.read_line(&mut input);
    let res = write_file(input);
    match res {
        Ok(k) => println!("file created succesfully {:?}", k),
        Err(e) => println!("nahh {}", e),
    }
    let x = Node::default(25);
    println!("{} {}", x.data, Node::square(&x));
}
