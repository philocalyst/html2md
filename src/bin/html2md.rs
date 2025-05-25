extern crate html2md;

use std::env::args;

fn main() {
    let argument = args().nth(1).unwrap();

    println!("{}", html2md::parse_html(&argument));
}
