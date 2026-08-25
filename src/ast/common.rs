use std::str::FromStr;

use pest::iterators::Pair;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::Rule;

#[derive(Debug, Clone)]
pub struct Verbatim {
    tokens: TokenStream,
}

impl Verbatim {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::verbatim = pair.as_rule() {
            Some(Self {
                tokens: TokenStream::from_str(
                    pair.into_inner()
                        .next()
                        .expect("verbatim always has inner_verbatim")
                        .to_string()
                        .as_str(),
                )
                .expect("could not parse verbatim"),
            })
        } else {
            None
        }
    }

    pub fn tokens(&self) -> &TokenStream {
        &self.tokens
    }
}

impl ToTokens for Verbatim {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let verbatim = self.tokens();

        tokens.extend(quote! { #verbatim });
    }
}
