use pest::iterators::Pair;

use crate::{
    Rule,
    ast::{insert::Insert, xclass::XClass},
};

pub mod common;
pub mod insert;
pub mod xclass;

#[derive(Debug, Clone)]
pub struct Interface {
    ccp_type: String,
    name: String,
    version: String,
    interface_elements: Vec<InterfaceElement>,
}

impl Interface {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::interface = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let ccp_type = pairs
                .next()
                .expect("interface always has interface_type")
                .to_string();
            let name = pairs.next().expect("interface always has name").to_string();
            let version = pairs
                .next()
                .expect("interface always has version")
                .to_string();

            let interface_elements = Self::parse_elements(
                pairs
                    .next()
                    .expect("interface always has interface_elements"),
            )
            .expect("could not parse interface_elements");

            Some(Self {
                ccp_type,
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
                    .map(|pair| {
                        InterfaceElement::parse(pair).expect("could not parse interface_element")
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn ccp_type(&self) -> &String {
        &self.ccp_type
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn version(&self) -> &String {
        &self.version
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
            Rule::insert => unimplemented!("insert elements are not implemented"), // Insert::parse(pair).map(Self::Insert),
            _ => None,
        }
    }
}
