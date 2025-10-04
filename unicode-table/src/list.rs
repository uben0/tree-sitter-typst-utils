use super::Entry;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "list.pest"]
pub struct ListParser;

fn parse_hex<'a>(pair: Pair<'a, Rule>) -> u32 {
    u32::from_str_radix(pair.as_str(), 16).unwrap()
}

fn parse_entry<'a>(pair: Pair<'a, Rule>) -> Entry<'a> {
    let mut children = pair.into_inner();
    let property = children.next_back().unwrap().as_str();
    let lhs = children.next().map(parse_hex).unwrap();
    let rhs = children.next().map(parse_hex).unwrap_or(lhs);
    Entry {
        range: lhs..=rhs,
        property,
    }
}

pub fn parse<'a>(content: &'a str) -> Vec<Entry<'a>> {
    let mut entries = ListParser::parse(Rule::file, content)
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .into_inner();
    entries.next_back();
    entries.map(parse_entry).collect()
}
