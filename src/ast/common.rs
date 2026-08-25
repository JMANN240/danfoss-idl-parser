use std::str::FromStr;

use pest::iterators::Pair;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::Rule;

#[derive(Debug, Clone)]
pub struct Verbatim(pub TokenStream);

impl Verbatim {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::verbatim = pair.as_rule() {
            Some(Self(
                TokenStream::from_str(pair.into_inner().next().unwrap().to_string().as_str())
                    .unwrap(),
            ))
        } else {
            None
        }
    }
}

impl ToTokens for Verbatim {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let verbatim = &self.0;
        tokens.extend(quote! { #verbatim });
    }
}
