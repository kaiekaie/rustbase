#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate pest;
use pest::{iterators::Pairs, Parser};
use serde::ser::Error;

#[derive(Parser)]
#[grammar = "filter_parser.pest"] // relative to src
pub struct FilterParser;
#[derive(Debug, Clone)]
enum Object {
    Request(RequestEnum),
    Collection(CollectionObject),
    Identifier(String),
    QuotedText(String),
    Number(i32),
}

#[derive(Debug, Clone)]
enum RequestEnum {
    AuthObject { key: String },
    HeaderObject { key: String },
}

#[derive(Debug, Clone)]
struct CollectionObject {
    name: String,
    column: String,
}
#[derive(Debug, Clone, Copy)]
enum Operator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    NotLike,
    Contains,
    NotContains,
    IsNull,
    NotNull,
}
#[derive(Debug, Clone)]
struct Expression {
    left: Object,
    op: Operator,
    right: Object,
}

#[derive(Debug, Clone)]
pub struct Statement {
    expressions: Vec<Expression>,
    join_operators: Vec<String>,
}

fn parse_statement(input: &str) -> Result<Statement, String> {
    let pairs = FilterParser::parse(Rule::statement, input);
    let mut expressions = Vec::new();
    let mut join_operators = Vec::new();
    match pairs {
        Ok(mut p) => {
            for pair in p.next().unwrap().into_inner() {
                match pair.as_rule() {
                    Rule::expression => {
                        let mut inner_pairs = pair.into_inner();

                        let left = parse_object(inner_pairs.next().unwrap());
                        let op = parse_operator(inner_pairs.next().unwrap().as_str());
                        let right = parse_right_expression(inner_pairs.next().unwrap());

                        expressions.push(Expression { left, op, right });
                    }
                    Rule::join_operator => {
                        join_operators.push(pair.as_str().to_string());
                    }
                    s => println!("{:?}", s),
                };
            }
            let statement = Statement {
                expressions,
                join_operators,
            };
            if statement.expressions.is_empty() && statement.join_operators.is_empty() {
                Err(String::from("nothing parsed"))
            } else {
                Ok(statement)
            }
        }
        Err(e) => Err(format!("{}", e)),
    }
}

fn parse_right_expression(pair: pest::iterators::Pair<Rule>) -> Object {
    match pair.as_rule() {
        Rule::identifier => Object::Identifier(String::from("adss")),
        Rule::quoted_text => Object::QuotedText(String::from("adss")),
        Rule::number => Object::Number(12),
        _ => unreachable!(),
    }
}

fn parse_object(pair_optional: pest::iterators::Pair<Rule>) -> Object {
    if pair_optional.clone().into_inner().next().is_some() {
        let pair_optional = pair_optional.into_inner().next();
        if let Some(pair) = pair_optional {
            match pair.as_rule() {
                Rule::request => {
                    let mut inner_pairs = pair.into_inner();
                    let r = inner_pairs.next().unwrap();

                    let auth = match r.as_rule() {
                        Rule::header => RequestEnum::HeaderObject {
                            key: parse_header_keys(r.as_str()),
                        },
                        Rule::auth => RequestEnum::AuthObject {
                            key: parse_auth_keys(r.as_str()),
                        },
                        s => unreachable!("{:?}", s),
                    };

                    Object::Request(auth)
                    //Object::Identifier(String::from(""))
                }
                Rule::collection => {
                    let mut inner_pairs = pair.into_inner();
                    let name = inner_pairs.next().unwrap().as_str().to_string();
                    let column = inner_pairs.next().unwrap().as_str().to_string();
                    Object::Collection(CollectionObject { name, column })
                }
                _ => unreachable!(),
            }
        } else {
            Object::Identifier(String::from(""))
        }
    } else {
        Object::Identifier(String::from(pair_optional.as_str()))
    }
}

fn parse_auth_keys(input: &str) -> String {
    match input {
        "auth.id" => "id".to_string(),
        "role.id" => "role".to_string(),
        _ => unreachable!(),
    }
}

fn parse_header_keys(input: &str) -> String {
    match input {
        "header.method" => "method".to_string(),
        "header.status" => "status".to_string(),
        _ => unreachable!(),
    }
}

fn parse_operator(input: &str) -> Operator {
    match input {
        "=" => Operator::Equal,
        "!=" => Operator::NotEqual,
        ">" => Operator::GreaterThan,
        ">=" => Operator::GreaterThanOrEqual,
        "<" => Operator::LessThan,
        "<=" => Operator::LessThanOrEqual,
        "~" => Operator::Like,
        "!~" => Operator::NotLike,
        "?=" => Operator::Contains,
        "?!=" => Operator::NotContains,
        "?>" => Operator::IsNull,
        "?>=" => Operator::IsNull,
        "?<" => Operator::NotNull,
        "?<=" => Operator::NotNull,
        "?~" => Operator::Like,
        "?!~" => Operator::NotLike,
        _ => unreachable!(),
    }
}

#[test]
fn test_filter() {
    let input = "@request.auth.id != 200 && poop = 'asds'";
    let output = parse_statement(input);
    let expressions = output.unwrap().clone();
    println!("{:?}", expressions);
    for expression in expressions.expressions {
        match expression.left {
            Object::Request(RequestEnum) => match RequestEnum {
                RequestEnum::AuthObject { key } => {
                    println!("{:?}", key);
                }
                RequestEnum::HeaderObject { key } => {
                    println!("{:?}", key);
                }
                _ => (),
            },
            Object::Identifier(String) => {}
            Object::QuotedText(String) => {}
            Object::Number(i32) => {}
            Object::Collection(CollectionObject) => {}
            _ => {}
        };
        match expression.right {
            Object::Request(RequestEnum) => {}
            Object::Identifier(String) => {}
            Object::QuotedText(String) => {}
            Object::Number(i32) => {}
            Object::Collection(CollectionObject) => {}
            _ => {}
        }
    }
}

#[test]
fn test_filter_fail() {
    let input2 = "@asd.sd.id != 200 && poop = 'asds'";
    let output = parse_statement(input2);

    assert!(!output.is_ok());
}
