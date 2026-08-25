use pest::iterators::Pair;
use quote::ToTokens;

use crate::{Rule, ast::common::Verbatim};

#[derive(Debug, Clone, Default)]
pub struct Insert {
    pub properties: Option<Vec<InsertProperty>>,
    pub verbatim: Option<Verbatim>,
}

impl Insert {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::insert = pair.as_rule() {
            let mut insert = Self::default();

            for pair in pair.into_inner() {
                match pair.as_rule() {
                    Rule::insert_properties => {
                        insert.properties = Self::parse_insert_properties(pair);
                    }
                    Rule::verbatim => {
                        insert.verbatim = Verbatim::parse(pair);
                    }
                    _ => (),
                }
            }

            Some(insert)
        } else {
            None
        }
    }

    pub fn parse_insert_properties(pair: Pair<Rule>) -> Option<Vec<InsertProperty>> {
        if let Rule::insert_properties = pair.as_rule() {
            Some(
                pair.into_inner()
                    .map(|pair| InsertProperty::parse(pair).unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }
}

impl ToTokens for Insert {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if let Some(verbatim) = &self.verbatim {
            verbatim.to_tokens(tokens);
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InsertProperty {
    pub name: String,
    pub value: String,
}

impl InsertProperty {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::insert_property = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let name = pairs.next().unwrap().to_string();
            let value = pairs
                .next()
                .unwrap()
                .into_inner()
                .next()
                .unwrap()
                .to_string();

            Some(Self { name, value })
        } else {
            None
        }
    }
}
