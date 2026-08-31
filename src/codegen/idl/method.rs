use capitalize::Capitalize;
use heck::ToSnakeCase;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use crate::{
    ast::xclass::{XClass, XClassProperty, method::Method, variable::Variable},
    codegen::{
        idl::instance_struct::InstanceStruct,
        rust::definition::{ArgumentDefiner, ArgumentDefinition, MethodDefiner},
    },
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

    fn return_type_tokens(&self) -> impl ToTokens {
        self.method().method_type()
    }

    fn arguments(&self) -> Vec<ArgumentDefinition> {
        let self_type_identifier = self.xclass().identifier();

        [ArgumentDefinition::new(
            format_ident!("{}", self_type_identifier.to_string().to_snake_case()),
            quote! { *mut #self_type_identifier },
        )]
        .into_iter()
        .chain(
            self.method()
                .arguments()
                .iter()
                .map(ArgumentDefiner::to_argument_definition),
        )
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

    fn return_type_tokens(&self) -> impl ToTokens {
        quote! { () }
    }

    fn arguments(&self) -> Vec<ArgumentDefinition> {
        let self_type_identifier = self.xclass().identifier();

        vec![ArgumentDefinition::new(
            format_ident!("{}", self_type_identifier.to_string().to_snake_case()),
            quote! { *mut #self_type_identifier },
        )]
    }

    fn body(&self) -> Option<impl ToTokens> {
        self.xclass_property()
            .maybe_verbatim()
            .map(ToTokens::to_token_stream)
    }
}

pub struct XClassVariableGetMethodDefinition {
    xclass: XClass,
    variable: Variable,
}

impl XClassVariableGetMethodDefinition {
    pub fn new(xclass: XClass, variable: Variable) -> Self {
        Self { xclass, variable }
    }

    pub fn xclass(&self) -> &XClass {
        &self.xclass
    }

    pub fn variable(&self) -> &Variable {
        &self.variable
    }

    pub fn self_type_identifier(&self) -> &Ident {
        self.xclass().identifier()
    }

    pub fn self_instance_identifier(&self) -> Ident {
        format_ident!(
            "{}",
            self.self_type_identifier().to_string().to_snake_case()
        )
    }
}

impl MethodDefiner for XClassVariableGetMethodDefinition {
    fn method_identifier(&self) -> Ident {
        format_ident!(
            "{}_{}GET",
            self.xclass().identifier(),
            self.variable()
                .identifier()
                .to_string()
                .capitalize_first_only(),
        )
    }

    fn return_type_tokens(&self) -> impl ToTokens {
        self.variable().variable_type()
    }

    fn arguments(&self) -> Vec<ArgumentDefinition> {
        let self_type_identifier = self.self_type_identifier();

        vec![ArgumentDefinition::new(
            self.self_instance_identifier(),
            quote! { *mut #self_type_identifier },
        )]
    }

    fn body(&self) -> Option<impl ToTokens> {
        let self_instance_identifier = self.self_instance_identifier();

        let self_instance_assignment = quote! {
            let #self_instance_identifier = unsafe { &mut *#self_instance_identifier }
        };

        let identifier = &self.variable().identifier();

        Some(if self.variable().is_shared() {
            let shared_field_identifier = InstanceStruct::shared_field_identifier();

            let shared_field_assignment = quote! {
                let #shared_field_identifier = unsafe { &mut *#self_instance_identifier.#shared_field_identifier }
            };

            quote! {
                #self_instance_assignment;
                #shared_field_assignment;
                #shared_field_identifier.#identifier
            }
        } else {

            quote! {
                #self_instance_assignment;
                #self_instance_identifier.#identifier
            }
        })
    }
}

pub struct XClassVariableSetMethodDefinition {
    xclass: XClass,
    variable: Variable,
}

impl XClassVariableSetMethodDefinition {
    pub fn new(xclass: XClass, variable: Variable) -> Self {
        Self { xclass, variable }
    }

    pub fn xclass(&self) -> &XClass {
        &self.xclass
    }

    pub fn variable(&self) -> &Variable {
        &self.variable
    }

    pub fn self_type_identifier(&self) -> &Ident {
        self.xclass().identifier()
    }

    pub fn self_instance_identifier(&self) -> Ident {
        format_ident!(
            "{}",
            self.self_type_identifier().to_string().to_snake_case()
        )
    }
}

impl MethodDefiner for XClassVariableSetMethodDefinition {
    fn method_identifier(&self) -> Ident {
        format_ident!(
            "{}_{}SET",
            self.xclass().identifier(),
            self.variable()
                .identifier()
                .to_string()
                .capitalize_first_only(),
        )
    }

    fn return_type_tokens(&self) -> impl ToTokens {
        quote! { () }
    }

    fn arguments(&self) -> Vec<ArgumentDefinition> {
        let self_type_identifier = self.self_type_identifier();
        let identifier = self.variable().identifier();
        let variable_type_tokens = self.variable().variable_type();

        vec![
            ArgumentDefinition::new(
                self.self_instance_identifier(),
                quote! { *mut #self_type_identifier },
            ),
            ArgumentDefinition::new(identifier.clone(), variable_type_tokens),
        ]
    }

    fn body(&self) -> Option<impl ToTokens> {
        let self_instance_identifier = self.self_instance_identifier();

        let self_instance_assignment = quote! {
            let #self_instance_identifier = unsafe { &mut *#self_instance_identifier }
        };

        let identifier = &self.variable().identifier();

        Some(if self.variable().is_shared() {
            let shared_field_identifier = InstanceStruct::shared_field_identifier();

            let shared_field_assignment = quote! {
                let #shared_field_identifier = unsafe { &mut *#self_instance_identifier.#shared_field_identifier }
            };

            quote! {
                #self_instance_assignment;
                #shared_field_assignment;
                #shared_field_identifier.#identifier = #identifier;
            }
        } else {

            quote! {
                #self_instance_assignment;
                #self_instance_identifier.#identifier = #identifier;
            }
        })
    }
}
