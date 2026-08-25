use pest::iterators::Pair;

use crate::{Rule, ast::{insert::Insert, xclass::XClass}};

pub mod common;
pub mod insert;
pub mod xclass;

#[derive(Debug, Clone)]
pub struct Interface {
    pub r#type: String,
    pub name: String,
    pub version: String,
    pub interface_elements: Vec<InterfaceElement>,
}

impl Interface {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::interface = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let r#type = pairs.next().unwrap().to_string();
            let name = pairs.next().unwrap().to_string();
            let version = pairs.next().unwrap().to_string();

            let interface_elements = Self::parse_elements(pairs.next().unwrap()).unwrap();

            Some(Self {
                r#type,
                name,
                version,
                interface_elements,
            })
        } else {
            None
        }
    }

    pub fn parse_elements(pair: Pair<Rule>) -> Option<Vec<InterfaceElement>> {
        if let Rule::interface_elements = pair.as_rule() {
            Some(
                pair.into_inner()
                    .map(|pair| InterfaceElement::parse(pair).unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn interface_elements(&self) -> &Vec<InterfaceElement> {
        &self.interface_elements
    }
}

#[derive(Debug, Clone)]
pub enum InterfaceElement {
    XClass(XClass),
    Insert(Insert),
}

impl InterfaceElement {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::xclass => XClass::parse(pair).map(Self::XClass),
            Rule::insert => Insert::parse(pair).map(Self::Insert),
            _ => None,
        }
    }
}
