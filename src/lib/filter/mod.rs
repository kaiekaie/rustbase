#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate pest;
use pest::{iterators::Pairs, Parser};
use serde::ser::Error;

#[derive(Parser)]
#[grammar = "filter_parser.pest"] // relative to src
pub struct FilterParser;

pub fn scan(input: &str) -> Result<Pairs<Rule>, pest::error::Error<Rule>> {
    FilterParser::parse(Rule::statement, input)
}
