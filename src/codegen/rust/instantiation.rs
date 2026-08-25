use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

pub struct StructInstantiation {
    identifier: Ident,
    fields: Vec<StructFieldInstantiation>,
}

impl StructInstantiation {
    pub fn new(identifier: Ident, fields: Vec<StructFieldInstantiation>) -> Self {
        Self { identifier, fields }
    }

    pub fn identifier(&self) -> &Ident {
        &self.identifier
    }

    pub fn fields(&self) -> &Vec<StructFieldInstantiation> {
        &self.fields
    }
}

impl ToTokens for StructInstantiation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let identifier = self.identifier();
        let fields = self.fields();

        tokens.extend(quote! {
            #identifier {
                #(#fields),*
            }
        });
    }
}

pub trait StructInstantiator {
    fn identifier(&self) -> Ident;
    fn fields(&self) -> Vec<StructFieldInstantiation>;

    fn to_struct_instantiation(&self) -> StructInstantiation {
        StructInstantiation::new(self.identifier(), self.fields())
    }
}

#[derive(Debug, Clone)]
pub struct StructFieldInstantiation {
    identifier: Ident,
    value_tokens: TokenStream,
}

impl StructFieldInstantiation {
    pub fn new(identifier: Ident, value_tokens: impl ToTokens) -> Self {
        Self {
            identifier,
            value_tokens: value_tokens.to_token_stream(),
        }
    }

    pub fn identifier(&self) -> &Ident {
        &self.identifier
    }

    pub fn value_tokens(&self) -> &TokenStream {
        &self.value_tokens
    }
}

impl ToTokens for StructFieldInstantiation {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let identifier = self.identifier();
        let value_tokens = self.value_tokens();

        tokens.extend(quote! { #identifier: #value_tokens });
    }
}

pub trait StructFieldInstantiator {
    fn identifier(&self) -> Ident;
    fn value_tokens(&self) -> impl ToTokens;

    fn to_struct_field_instantiation(&self) -> StructFieldInstantiation {
        StructFieldInstantiation::new(self.identifier(), self.value_tokens())
    }
}

#[derive(Debug, Clone)]
pub struct StaticAssignment {
    identifier: Ident,
    type_tokens: TokenStream,
    value_tokens: TokenStream,
}

impl StaticAssignment {
    pub fn new(identifier: Ident, type_tokens: impl ToTokens, value_tokens: impl ToTokens) -> Self {
        Self {
            identifier,
            type_tokens: type_tokens.to_token_stream(),
            value_tokens: value_tokens.to_token_stream(),
        }
    }

    pub fn identifier(&self) -> &Ident {
        &self.identifier
    }

    pub fn type_tokens(&self) -> &TokenStream {
        &self.type_tokens
    }

    pub fn value_tokens(&self) -> &TokenStream {
        &self.value_tokens
    }
}

impl ToTokens for StaticAssignment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let identifier = self.identifier();
        let type_tokens = self.type_tokens();
        let value_tokens = self.value_tokens();

        tokens.extend(quote! {
            #[unsafe(no_mangle)]
            pub static mut #identifier: #type_tokens = #value_tokens;
        });
    }
}

pub trait StaticAssigner {
    fn identifier(&self) -> Ident;
    fn type_tokens(&self) -> impl ToTokens;
    fn value_tokens(&self) -> impl ToTokens;

    fn to_static_assignment(&self) -> StaticAssignment {
        StaticAssignment::new(
            self.identifier(),
            self.type_tokens(),
            self.value_tokens(),
        )
    }
}
