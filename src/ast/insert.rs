use pest::iterators::Pair;
use quote::{ToTokens, quote};

use crate::{Rule, ast::common::Verbatim};

#[derive(Debug, Clone, Default)]
pub struct Insert {
    maybe_properties: Option<Vec<InsertProperty>>,
    maybe_verbatim: Option<Verbatim>,
}

impl Insert {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::insert = pair.as_rule() {
            let mut insert = Self::default();

            for pair in pair.into_inner() {
                match pair.as_rule() {
                    Rule::insert_properties => {
                        insert.maybe_properties = Self::parse_insert_properties(pair);
                    }
                    Rule::verbatim => {
                        insert.maybe_verbatim = Verbatim::parse(pair);
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
                    .map(|pair| InsertProperty::parse(pair).expect("coult not parse insert property"))
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn maybe_properties(&self) -> Option<&Vec<InsertProperty>> {
        self.maybe_properties.as_ref()
    }

    pub fn maybe_verbatim(&self) -> Option<&Verbatim> {
        self.maybe_verbatim.as_ref()
    }
}

impl ToTokens for Insert {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let maybe_verbatim = self.maybe_verbatim();

        tokens.extend(quote! { #maybe_verbatim });
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InsertProperty {
    name: String,
    value: String,
}

impl InsertProperty {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::insert_property = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let name = pairs.next().expect("insert_property always has name").to_string();
            let value = pairs
                .next()
                .expect("insert_property always has string")
                .into_inner()
                .next()
                .expect("string always has inner_string")
                .to_string();

            Some(Self { name, value })
        } else {
            None
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}
