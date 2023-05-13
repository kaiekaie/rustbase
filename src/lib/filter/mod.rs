#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(dead_code)]

use mongodb::bson;
use pest::iterators::Pair;
extern crate pest;

#[derive(Parser)]
#[grammar = "filter_parser.pest"]
pub struct FilterParser;

fn parse_filter(filter: &str) -> Result<Expression, pest::error::Error<Rule>> {
    let parsed = FilterParser::parse(Rule::filter, filter)?.next().unwrap();
    Ok(parse_expression(parsed))
}

fn parse_expression(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::or_expression => {
            let mut pairs = pair.into_inner();
            let lhs = parse_and_expression(pairs.next().unwrap());
            let rhs = pairs.map(parse_and_expression).collect::<Vec<_>>();
            Expression::Or(Box::new(lhs), rhs)
        }
        _ => parse_and_expression(pair),
    }
}

fn parse_and_expression(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::and_expression => {
            let mut pairs = pair.into_inner();
            let lhs = parse_atom_expression(pairs.next().unwrap());
            let rhs = pairs.map(parse_atom_expression).collect::<Vec<_>>();
            Expression::And(Box::new(lhs), rhs)
        }
        _ => parse_atom_expression(pair),
    }
}

fn parse_atom_expression(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::field_expression => Expression::Field(pair.as_str().to_string()),
        Rule::value_expression => Expression::Value(pair.as_str().parse().unwrap()),
        Rule::expression => parse_expression(pair.into_inner().next().unwrap()),
        _ => unreachable!(),
    }
}

enum Expression {
    Or(Box<Expression>, Vec<Expression>),
    And(Box<Expression>, Vec<Expression>),
    Field(String),
    Value(i64),
}

struct Filter {
    expression: Expression,
}

impl Filter {
    fn to_mongodb(&self) -> bson::Document {
        let mut doc = bson::Document::new();
        self.expression.to_mongodb(&mut doc);
        doc
    }
}

impl Expression {
    fn to_mongodb(&self, doc: &mut bson::Document) {
        match self {
            Expression::Or(lhs, rhs) => {
                let mut arr = bson::Array::new();
                lhs.to_mongodb(doc);
                for expr in rhs {
                    let mut child_doc = bson::Document::new();
                    expr.to_mongodb(&mut child_doc);
                    arr.push(child_doc);
                }
                doc.insert("$or", arr);
            }
            Expression::And(lhs, rhs) => {
                let mut arr = bson::Array::new();
                lhs.to_mongodb(doc);
                for expr in rhs {
                    let mut child_doc = bson::Document::new();
                    expr.to_mongodb(&mut child_doc);
                    arr.push(child_doc);
                }
                doc.insert("$and", arr);
            }
            Expression::Field(name) => {
                doc.insert(name, bson::Bson::Int64(1));
            }
            Expression::Value(value) => {
                doc.insert("$eq", bson::Bson::Int64(*value));
            }
        }
    }
}
