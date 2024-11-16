use std::fs;
mod ast;
mod parser;

fn main() {
    let content = fs::read_to_string("CHANGELOG.md").unwrap();
    let (remaining, changelog) = parser::parse_changelog(&content).unwrap();
    println!("{:#?}", changelog);
    println!("{:?}", remaining)
}
