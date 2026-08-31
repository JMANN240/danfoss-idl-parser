use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::Ident;

pub struct StructDefinition {
    identifier: Ident,
    fields: Vec<StructFieldDefinition>,
}

impl StructDefinition {
    pub fn new(identifier: Ident, fields: Vec<StructFieldDefinition>) -> Self {
        Self { identifier, fields }
    }

    pub fn identifier(&self) -> &Ident {
        &self.identifier
    }

    pub fn fields(&self) -> &Vec<StructFieldDefinition> {
        &self.fields
    }
}

impl ToTokens for StructDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let identifier = self.identifier();
        let fields = self.fields();

        tokens.extend(quote! {
            #[repr(C)]
            pub struct #identifier {
                #(#fields),*
            }
        });
    }
}

pub trait StructDefiner {
    fn identifier(&self) -> Ident;
    fn fields(&self) -> Vec<StructFieldDefinition>;

    fn to_struct_definition(&self) -> StructDefinition {
        StructDefinition::new(self.identifier(), self.fields())
    }
}

#[derive(Debug, Clone)]
pub struct StructFieldDefinition {
    identifier: Ident,
    type_tokens: TokenStream,
}

impl StructFieldDefinition {
    pub fn new(identifier: Ident, type_tokens: impl ToTokens) -> Self {
        Self {
            identifier,
            type_tokens: type_tokens.to_token_stream(),
        }
    }

    pub fn identifier(&self) -> &Ident {
        &self.identifier
    }

    pub fn type_tokens(&self) -> &TokenStream {
        &self.type_tokens
    }
}

impl ToTokens for StructFieldDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let identifier = self.identifier();
        let type_tokens = self.type_tokens();

        tokens.extend(quote! { pub #identifier: #type_tokens });
    }
}

pub trait StructFieldDefiner {
    fn identifier(&self) -> Ident;
    fn type_tokens(&self) -> impl ToTokens;

    fn to_struct_field_definition(&self) -> StructFieldDefinition {
        StructFieldDefinition::new(self.identifier(), self.type_tokens())
    }
}

pub struct MethodDefinition {
    method_identifier: Ident,
    return_type_tokens: TokenStream,
    arguments: Vec<ArgumentDefinition>,
    body: Option<TokenStream>,
}

impl MethodDefinition {
    pub fn new(
        method_identifier: Ident,
        return_type_tokens: impl ToTokens,
        arguments: Vec<ArgumentDefinition>,
        body: Option<impl ToTokens>,
    ) -> Self {
        Self {
            method_identifier,
            return_type_tokens: return_type_tokens.to_token_stream(),
            arguments,
            body: body.as_ref().map(ToTokens::to_token_stream),
        }
    }

    pub fn method_identifier(&self) -> &Ident {
        &self.method_identifier
    }

    pub fn return_type_tokens(&self) -> &TokenStream {
        &self.return_type_tokens
    }

    pub fn arguments(&self) -> &Vec<ArgumentDefinition> {
        &self.arguments
    }

    pub fn body(&self) -> Option<&TokenStream> {
        self.body.as_ref()
    }
}

impl ToTokens for MethodDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let method_identifier = self.method_identifier();
        let return_type_tokens = self.return_type_tokens();
        let arguments = self.arguments();

        let body = if let Some(body) = self.body() {
            body.clone()
        } else {
            quote! { todo!(); }
        };

        tokens.extend(quote! {
            #[unsafe(no_mangle)]
            pub extern "C" fn #method_identifier(
                #(#arguments),*
            ) -> #return_type_tokens {
                #body
            }
        });
    }
}

pub trait MethodDefiner {
    fn method_identifier(&self) -> Ident;
    fn return_type_tokens(&self) -> impl ToTokens;
    fn arguments(&self) -> Vec<ArgumentDefinition>;
    fn body(&self) -> Option<impl ToTokens>;

    fn to_method_definition(&self) -> MethodDefinition {
        MethodDefinition::new(
            self.method_identifier(),
            self.return_type_tokens(),
            self.arguments(),
            self.body(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct ArgumentDefinition {
    identifier: Ident,
    type_tokens: TokenStream,
}

impl ArgumentDefinition {
    pub fn new(identifier: Ident, type_tokens: impl ToTokens) -> Self {
        Self {
            identifier,
            type_tokens: type_tokens.to_token_stream(),
        }
    }

    pub fn identifier(&self) -> &Ident {
        &self.identifier
    }

    pub fn type_tokens(&self) -> &TokenStream {
        &self.type_tokens
    }
}

impl ToTokens for ArgumentDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let identifier = self.identifier();
        let type_tokens = self.type_tokens();

        tokens.extend(quote! { #identifier: #type_tokens });
    }
}

pub trait ArgumentDefiner {
    fn identifier(&self) -> Ident;
    fn type_tokens(&self) -> impl ToTokens;

    fn to_argument_definition(&self) -> ArgumentDefinition {
        ArgumentDefinition::new(self.identifier(), self.type_tokens())
    }
}
