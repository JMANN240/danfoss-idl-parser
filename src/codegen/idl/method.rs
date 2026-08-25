use capitalize::Capitalize;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use crate::{
    ast::xclass::{XClass, XClassProperty, method::Method, variable::Variable},
    codegen::rust::definition::{ArgumentDefiner, ArgumentDefinition, MethodDefiner},
};

pub struct XClassMethodDefinition {
    xclass: XClass,
    method: Method,
}

impl XClassMethodDefinition {
    pub fn new(xclass: XClass, method: Method) -> Self {
        Self { xclass, method }
    }

    pub fn xclass(&self) -> &XClass {
        &self.xclass
    }

    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl MethodDefiner for XClassMethodDefinition {
    fn method_identifier(&self) -> Ident {
        format_ident!(
            "{}_{}",
            self.xclass().identifier(),
            self.method().identifier()
        )
    }

    fn self_type_tokens(&self) -> impl ToTokens {
        let self_type_identifier = self.xclass().identifier();

        quote! { *mut #self_type_identifier }
    }

    fn return_type_tokens(&self) -> impl ToTokens {
        self.method().method_type()
    }

    fn arguments(&self) -> Vec<ArgumentDefinition> {
        self.method()
            .arguments()
            .iter()
            .map(ArgumentDefiner::to_argument_definition)
            .collect()
    }

    fn body(&self) -> Option<impl ToTokens> {
        let verbatims = self.method().verbatims().collect::<Vec<_>>();

        (!verbatims.is_empty()).then(|| {
            quote! { #(#verbatims)* }
        })
    }
}

pub struct XClassPropertyMethodDefinition {
    xclass: XClass,
    xclass_property: XClassProperty,
}

impl XClassPropertyMethodDefinition {
    pub fn new(xclass: XClass, xclass_property: XClassProperty) -> Self {
        Self {
            xclass,
            xclass_property,
        }
    }

    pub fn xclass(&self) -> &XClass {
        &self.xclass
    }

    pub fn xclass_property(&self) -> &XClassProperty {
        &self.xclass_property
    }
}

impl MethodDefiner for XClassPropertyMethodDefinition {
    fn method_identifier(&self) -> Ident {
        format_ident!(
            "{}_{}",
            self.xclass().identifier(),
            self.xclass_property().name()
        )
    }

    fn self_type_tokens(&self) -> impl ToTokens {
        let self_type_identifier = self.xclass().identifier();

        quote! { *mut #self_type_identifier }
    }

    fn return_type_tokens(&self) -> impl ToTokens {
        quote! { () }
    }

    fn arguments(&self) -> Vec<ArgumentDefinition> {
        vec![]
    }

    fn body(&self) -> Option<impl ToTokens> {
        self.xclass_property()
            .verbatim()
            .map(ToTokens::to_token_stream)
    }
}

pub struct XClassVariableGetMethodDefinition {
    xclass: XClass,
    varaible: Variable,
}

impl XClassVariableGetMethodDefinition {
    pub fn new(xclass: XClass, varaible: Variable) -> Self {
        Self { xclass, varaible }
    }

    pub fn xclass(&self) -> &XClass {
        &self.xclass
    }

    pub fn varaible(&self) -> &Variable {
        &self.varaible
    }
}

impl MethodDefiner for XClassVariableGetMethodDefinition {
    fn method_identifier(&self) -> Ident {
        format_ident!(
            "{}_{}GET",
            self.xclass().identifier(),
            self.varaible()
                .identifier()
                .to_string()
                .capitalize_first_only(),
        )
    }

    fn self_type_tokens(&self) -> impl ToTokens {
        let self_type_identifier = self.xclass().identifier();

        quote! { *mut #self_type_identifier }
    }

    fn return_type_tokens(&self) -> impl ToTokens {
        self.varaible().variable_type()
    }

    fn arguments(&self) -> Vec<ArgumentDefinition> {
        vec![]
    }

    fn body(&self) -> Option<impl ToTokens> {
        let identifier = &self.varaible().identifier();

        Some(if self.varaible().is_shared() {
            quote! { self.SHARED.#identifier }
        } else {
            quote! { self.#identifier }
        })
    }
}

pub struct XClassVariableSetMethodDefinition {
    xclass: XClass,
    varaible: Variable,
}

impl XClassVariableSetMethodDefinition {
    pub fn new(xclass: XClass, varaible: Variable) -> Self {
        Self { xclass, varaible }
    }

    pub fn xclass(&self) -> &XClass {
        &self.xclass
    }

    pub fn varaible(&self) -> &Variable {
        &self.varaible
    }
}

impl MethodDefiner for XClassVariableSetMethodDefinition {
    fn method_identifier(&self) -> Ident {
        format_ident!(
            "{}_{}SET",
            self.xclass().identifier(),
            self.varaible()
                .identifier()
                .to_string()
                .capitalize_first_only(),
        )
    }

    fn self_type_tokens(&self) -> impl ToTokens {
        let self_type_identifier = self.xclass().identifier();

        quote! { *mut #self_type_identifier }
    }

    fn return_type_tokens(&self) -> impl ToTokens {
        quote! { () }
    }

    fn arguments(&self) -> Vec<ArgumentDefinition> {
        let identifier = self.varaible().identifier();
        let varaible_type_tokens = self.varaible().variable_type();

        vec![ArgumentDefinition::new(
            identifier.clone(),
            varaible_type_tokens,
        )]
    }

    fn body(&self) -> Option<impl ToTokens> {
        let identifier = self.varaible().identifier().clone();

        let value_tokens = if self.varaible().is_shared() {
            quote! { self.SHARED.#identifier }
        } else {
            quote! { self.#identifier }
        };

        Some(quote! { #value_tokens = #identifier; })
    }
}
