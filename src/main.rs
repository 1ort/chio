use std::fs;
mod ast;
mod fsm_parser;
mod parser;

fn main() {
    let content = fs::read_to_string("test-changelog.md").unwrap();
    let (remaining, changelog) = parser::parse_changelog(&content).unwrap();
    println!("{:#?}", changelog);
    println!("{:?}", remaining)
}
