use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{ast::xclass::XClass, codegen::rust::definition::StructDefiner};

pub struct AlienStruct {
    xclass: XClass,
}

impl AlienStruct {
    pub fn new(xclass: XClass) -> Self {
        Self { xclass }
    }

    pub fn xclass(&self) -> &XClass {
        &self.xclass
    }

    pub fn type_identifier(&self) -> Ident {
        format_ident!("{}Alien", self.xclass().identifier(),)
    }
}

impl StructDefiner for AlienStruct {
    fn identifier(&self) -> Ident {
        self.type_identifier()
    }

    fn field_tokens(&self) -> TokenStream {
        let verbatims = self.xclass().verbatims().collect::<Vec<_>>();

        quote! {
            #(#verbatims)*
        }
    }
}
