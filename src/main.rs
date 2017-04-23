use std::fs::File;
use std::io::{Read, stdin};
use std::env::args;
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() >= 2 {
        for name in &args[1..] {
            let mut f = File::open(name).unwrap();
            let mut result = String::new();
            f.read_to_string(&mut result);
            println!("{}", result);
        }
    } else {
        let mut line = String::new();
        stdin().read_to_string(&mut line);
        println!("{}", line);
    }
}
