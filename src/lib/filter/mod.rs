#![allow(clippy::all)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate pest;
use std::{collections::HashMap, f32::consts::E};

use mongodb::{
    bson::{self, doc, Bson, Document},
    Collection, Database,
};
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use r2d2::State;
use serde::ser::Error;
use testcontainers::{clients, images::mongo::Mongo};

use crate::models::collection;

use super::jwt::Jwt;

#[derive(Parser)]
#[grammar = "filter_parser.pest"] // relative to src
pub struct FilterParser;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Request(RequestEnum),
    Collection(CollectionObject),
    Identifier(String),
    QuotedText(String),
    Number(i32),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RequestEnum {
    AuthObject(Authkeys),
    HeaderObject(HeaderKeys),
    BodyObject(BodyObject),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Authkeys {
    Id,
    Role,
}
#[derive(Debug, Clone, PartialEq)]
pub enum HeaderKeys {
    Status,
    Method,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CollectionObject {
    name: String,
    column: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BodyObject {
    column: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Auth {
    id: String,
    role: String,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FakeHeader {
    pub method: String,
    pub status: i32,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
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
}
impl Operator {
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
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub left: Object,
    pub op: Operator,
    pub right: Object,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub expressions: Vec<Expression>,
    pub join_operators: Vec<JoinOperator>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum JoinOperator {
    And,
    Or,
}
impl JoinOperator {
    fn check_operator(join_operator: JoinOperator, boolean_left: bool, bool_right: bool) -> bool {
        match join_operator {
            JoinOperator::And => boolean_left && bool_right,
            JoinOperator::Or => boolean_left || bool_right,
        }
    }
    fn from_string(string: &str) -> JoinOperator {
        match string {
            "&&" => JoinOperator::And,
            "||" => JoinOperator::Or,
            &_ => unreachable!(""),
        }
    }
}

#[derive(Debug)]
pub struct Filter;

impl Filter {
    pub fn input_to_statment(input: &str) -> Result<Statement, String> {
        let pairs = FilterParser::parse(Rule::statement, input);
        let mut expressions = Vec::new();
        let mut join_operators = Vec::new();
        match pairs {
            Ok(mut p) => {
                for pair in p.next().unwrap().into_inner() {
                    match pair.as_rule() {
                        Rule::expression => {
                            let mut inner_pairs = pair.into_inner();
                            let mut left = parse_expression(inner_pairs.next().unwrap());
                            let op = Operator::parse_operator(inner_pairs.next().unwrap().as_str());
                            let mut right = parse_expression(inner_pairs.next().unwrap());
                            match left {
                                Object::Request(_) => match right {
                                    Object::Identifier(_) => {
                                        let mut temp = Object::Null;
                                        temp = left;
                                        left = right;
                                        right = temp;
                                    }
                                    _ => {}
                                },
                                Object::Collection(_) => match right {
                                    Object::Identifier(_) => {
                                        let mut temp = Object::Null;
                                        temp = left;
                                        left = right;
                                        right = temp;
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }
                            expressions.push(Expression { left, op, right });
                        }
                        Rule::join_operator => {
                            join_operators.push(JoinOperator::from_string(pair.as_str()));
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

    /*     pub async fn statement_operation(&mut self, input: &str) -> bool {
        let statement = Filter::input_to_statment(input).unwrap();
        self.statement = Some(statement);
        self.validate_statement().await
    } */

    /*     pub async fn validate_statement(&self) -> bool {
           let mut map = Vec::new();
           if let Some(statement) = &self.statement {
               for element in statement.expressions.iter() {
                   let left_value = self.replace_object(element.left.clone()).await;
                   let operator = element.op;
                   let right_value = self.replace_object(element.right.clone()).await;
                   let bools = values_checker(left_value, operator, right_value);
                   map.push(bools);
               }

               if !statement.join_operators.is_empty() {
                   return statement
                       .join_operators
                       .iter()
                       .zip(map.iter().enumerate())
                       .fold(map[0], |acc, (&op, (i, _))| {
                           let is_last = i == map.len() - 1;
                           if is_last {
                               JoinOperator::check_operator(op, acc, map[i])
                           } else {
                               JoinOperator::check_operator(op, map[i], map[i + 1])
                           }
                       });
               }
           }
           map.iter().all(|&bool| bool)
       }
    */
    fn bson_to_value(&self, document: Document, name: String) -> Value {
        if let Some(bson) = document.get(name) {
            match bson {
                bson::Bson::String(s) => Value::String(s.clone()),
                bson::Bson::Int32(s) => Value::Number(s.clone()),
                bson::Bson::Boolean(s) => Value::Boolean(s.clone()),
                _ => Value::None(),
            }
        } else {
            Value::None()
        }
    }

    /*     pub async fn replace_object(&self, object: Object) -> Value {
        match object {
            Object::Request(e) => match e {
                RequestEnum::AuthObject(auth_keys) => match auth_keys {
                    Authkeys::Id => Value::String(self.auth.get("id").unwrap().to_string()),
                    Authkeys::Role => Value::String(self.auth.get("role").unwrap().to_string()),
                },
                RequestEnum::HeaderObject(header_keys) => match header_keys {
                    HeaderKeys::Method => Value::String(self.header.method.clone()),
                    HeaderKeys::Status => Value::Number(self.header.status.clone()),
                },
                RequestEnum::BodyObject(_) => todo!(),
            },
            Object::Collection(s) => {
                let column = s.column.clone();
                let name = s.name.clone();
                let doc_collection: Collection<Document> = self.database.collection(&name);
                let document_optional = doc_collection
                    .find_one(Some(doc! {column: name}), None)
                    .await
                    .unwrap();
                if let Some(document) = document_optional {
                    return self.bson_to_value(document, s.column);
                }
                Value::None()
            }
            Object::Identifier(_s) => {
                todo!("");
                Value::None()
            }
            Object::QuotedText(s) => Value::String(s),
            Object::Number(s) => Value::Number(s),
            Object::Null => Value::None(),
        }
    } */
}

fn parse_expression(pair: pest::iterators::Pair<Rule>) -> Object {
    match pair.as_rule() {
        Rule::fixed_object => parse_object(pair),
        Rule::identifier => Object::Identifier(format!("{}", { pair.as_str() })),
        Rule::quoted_text => Object::QuotedText(format!("{}", { pair.as_str().replace("'", "") })),
        Rule::number => Object::Number(pair.as_str().parse::<i32>().unwrap()),
        Rule::null => Object::Null,
        r => unreachable!("{:?}", r),
    }
}

fn parse_object(pair: Pair<Rule>) -> Object {
    if let Some(pair) = pair.into_inner().next() {
        match pair.as_rule() {
            Rule::request => Object::Request(parse_request(pair.into_inner().next().unwrap())),
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
}

fn parse_request(input: Pair<Rule>) -> RequestEnum {
    match input.as_rule() {
        Rule::header => RequestEnum::HeaderObject(parse_header_keys(input)),
        Rule::auth => RequestEnum::AuthObject(parse_auth_keys(input)),
        Rule::body => {
            let mut inner_pairs = input.into_inner();
            let column = inner_pairs.next().unwrap().as_str().to_string();
            RequestEnum::BodyObject(BodyObject { column })
        }
        s => unreachable!("{:?}", s),
    }
}

fn parse_auth_keys(input: Pair<Rule>) -> Authkeys {
    match input.as_str() {
        "auth.id" => Authkeys::Id,
        "auth.role" => Authkeys::Role,
        _ => unreachable!(),
    }
}

fn parse_header_keys(input: Pair<Rule>) -> HeaderKeys {
    match input.as_str() {
        "header.method" => HeaderKeys::Method,
        "header.status" => HeaderKeys::Status,
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    String(String),
    Number(i32),
    Boolean(bool),
    None(),
}

pub fn values_checker(value: Value, op: Operator, value2: Value) -> bool {
    match op {
        Operator::Equal => value == value2,
        Operator::NotEqual => value != value2,
        Operator::GreaterThan => value > value2,
        Operator::GreaterThanOrEqual => value >= value2,
        Operator::LessThan => value < value2,
        Operator::LessThanOrEqual => value <= value2,
        Operator::Like => todo!(),
        Operator::NotLike => todo!(),
        Operator::Contains => todo!(),
        Operator::NotContains => todo!(),
    }
}

#[test]
fn test_parse_statement_empty() {
    let map: Vec<bool> = vec![false, false];
    let join_operators = vec![JoinOperator::Or];
    let result =
        join_operators
            .iter()
            .zip(map.iter().enumerate())
            .fold(map[0], |acc, (&op, (i, _))| {
                let is_last = i == map.len() - 1;
                if is_last {
                    JoinOperator::check_operator(op, acc, map[i])
                } else {
                    JoinOperator::check_operator(op, map[i], map[i + 1])
                }
            });

    println!("hello {:?}", result);
}
#[test]
fn test_parse_request_body() {
    let input = "@request.body.tester = 'adaw'";
    let expected_expression = Expression {
        left: Object::Request(RequestEnum::BodyObject(BodyObject {
            column: String::from("tester"),
        })),
        op: Operator::Equal,
        right: Object::QuotedText(String::from("adaw")),
    };
    let expected_statement = Statement {
        expressions: vec![expected_expression],
        join_operators: vec![],
    };
    assert_eq!(Filter::input_to_statment(input), Ok(expected_statement));
}

#[test]
fn test_parse_statement_single_expression() {
    let input = "@request.header.status = 200";
    let expected_expression = Expression {
        left: Object::Request(RequestEnum::HeaderObject(HeaderKeys::Status)),
        op: Operator::Equal,
        right: Object::Number(200),
    };
    let expected_statement = Statement {
        expressions: vec![expected_expression],
        join_operators: vec![],
    };
    assert_eq!(Filter::input_to_statment(input), Ok(expected_statement));
}

#[test]
fn test_parse_statement_multiple_expressions() {
    let input = "@request.header.status = 200 && @request.auth.role = 'admin'";
    let expected_expression_1 = Expression {
        left: Object::Request(RequestEnum::HeaderObject(HeaderKeys::Status)),
        op: Operator::Equal,
        right: Object::Number(200),
    };
    let expected_expression_2 = Expression {
        left: Object::Request(RequestEnum::AuthObject(Authkeys::Role)),
        op: Operator::Equal,
        right: Object::QuotedText(String::from("admin")),
    };

    let expected_statement = Statement {
        expressions: vec![expected_expression_1, expected_expression_2],
        join_operators: vec![JoinOperator::And],
    };
    assert_eq!(Filter::input_to_statment(input), Ok(expected_statement));
}

#[test]
fn test_parse_statement_single() {
    let input = "user_id = 200";
    let expected_expression = Expression {
        left: Object::Identifier(String::from("user_id")),
        op: Operator::Equal,
        right: Object::Number(200),
    };

    let expected_statement = Statement {
        expressions: vec![expected_expression],
        join_operators: vec![],
    };
    assert_eq!(Filter::input_to_statment(input), Ok(expected_statement));
}

#[test]
fn test_parse_statement_object_to_identifier() {
    let input = "@request.auth.id = user_id";
    let expected_expression = Expression {
        left: Object::Identifier(String::from("user_id")),
        op: Operator::Equal,
        right: Object::Request(RequestEnum::AuthObject(Authkeys::Id)),
    };

    let expected_statement = Statement {
        expressions: vec![expected_expression],
        join_operators: vec![],
    };
    assert_eq!(Filter::input_to_statment(input), Ok(expected_statement));
}
