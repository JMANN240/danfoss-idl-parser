use pest::iterators::Pair;
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use strum::EnumIs;
use syn::Ident;

use crate::{
    Rule,
    ast::{
        common::Verbatim,
        xclass::VariableType,
    },
    codegen::rust::{
        definition::{
            ArgumentDefiner, StructDefiner, StructFieldDefiner,
            StructFieldDefinition,
        },
        instantiation::{StructFieldInstantiator, StructInstantiator},
    },
};

#[derive(Debug, Clone)]
pub struct Method {
    pub method_type: MethodType,
    pub identifier: Ident,
    pub arguments: Vec<Argument>,
    pub elements: Vec<MethodElement>,
}

impl Method {
    pub fn identifier(&self) -> &Ident {
        &self.identifier
    }

    pub fn method_type(&self) -> MethodType {
        self.method_type
    }

    pub fn arguments(&self) -> &Vec<Argument> {
        &self.arguments
    }

    pub fn elements(&self) -> &Vec<MethodElement> {
        &self.elements
    }

    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::method = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let method_type = MethodType::parse(pairs.next().unwrap()).unwrap();
            let identifier = format_ident!("{}", pairs.next().unwrap().to_string());
            let arguments = Self::parse_arguments(pairs.next().unwrap()).unwrap();
            let elements = Self::parse_elements(pairs.next().unwrap()).unwrap();

            Some(Self {
                method_type,
                identifier,
                arguments,
                elements,
            })
        } else {
            None
        }
    }

    pub fn parse_arguments(pair: Pair<Rule>) -> Option<Vec<Argument>> {
        if let Rule::arg_list = pair.as_rule() {
            let mut current_default_argument_property = ArgumentProperty::In;

            Some(
                pair.into_inner()
                    .map(|pair| {
                        let argument =
                            Argument::parse(pair, current_default_argument_property).unwrap();

                        if argument.property.is_out() {
                            current_default_argument_property = ArgumentProperty::Out;
                        }

                        argument
                    })
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn parse_elements(pair: Pair<Rule>) -> Option<Vec<MethodElement>> {
        if let Rule::method_elements = pair.as_rule() {
            Some(
                pair.into_inner()
                    .map(|pair| MethodElement::parse(pair).unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn in_arguments(&self) -> impl Iterator<Item = &Argument> {
        self.arguments
            .iter()
            .filter(|argument| argument.property.is_in())
    }

    pub fn has_in_arguments(&self) -> bool {
        self.in_arguments().peekable().peek().is_some()
    }

    pub fn out_arguments(&self) -> impl Iterator<Item = &Argument> {
        self.arguments
            .iter()
            .filter(|argument| argument.property.is_out())
    }

    pub fn has_out_arguments(&self) -> bool {
        self.out_arguments().peekable().peek().is_some()
    }

    pub fn verbatims(&self) -> impl Iterator<Item = &Verbatim> {
        self.elements().iter().filter_map(|method_element| {
            if let MethodElement::Verbatim(verbatim) = method_element {
                Some(verbatim)
            } else {
                None
            }
        })
    }
}

impl StructDefiner for Method {
    fn identifier(&self) -> Ident {
        format_ident!("{}_OUT_ARGUMENTS_TYPE", self.identifier())
    }

    fn fields(&self) -> Vec<StructFieldDefinition> {
        self.out_arguments()
            .map(StructFieldDefiner::to_struct_field_definition)
            .collect()
    }
}

impl StructFieldDefiner for Method {
    fn identifier(&self) -> Ident {
        self.identifier().clone()
    }

    fn type_tokens(&self) -> impl ToTokens {
        StructDefiner::identifier(self)
    }
}

impl StructInstantiator for Method {
    fn identifier(&self) -> Ident {
        self.to_struct_definition().identifier().clone()
    }

    fn fields(&self) -> Vec<crate::codegen::rust::instantiation::StructFieldInstantiation> {
        self.out_arguments()
            .map(StructFieldInstantiator::to_struct_field_instantiation)
            .collect()
    }
}

impl StructFieldInstantiator for Method {
    fn identifier(&self) -> Ident {
        self.identifier().clone()
    }

    fn value_tokens(&self) -> impl ToTokens {
        self.to_struct_instantiation()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MethodType {
    VariableType(VariableType),
    Void,
}

impl MethodType {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::method_type = pair.as_rule() {
            if let Some(pair) = pair.into_inner().next() {
                VariableType::parse(pair).map(Self::VariableType)
            } else {
                Some(Self::Void)
            }
        } else {
            None
        }
    }
}

impl ToTokens for MethodType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::VariableType(variable_type) => variable_type.to_tokens(tokens),
            Self::Void => tokens.extend(quote! { () }),
        };
    }
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub variable_type: VariableType,
    pub identifier: Ident,
    pub maybe_initial_value: Option<String>,
    pub property: ArgumentProperty,
}

impl Argument {
    pub fn parse(pair: Pair<Rule>, default_argument_property: ArgumentProperty) -> Option<Self> {
        if let Rule::argument = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let variable_type = VariableType::parse(pairs.next().unwrap()).unwrap();
            let identifier = format_ident!("{}", pairs.next().unwrap().to_string());

            let mut maybe_initial_value_or_properties = pairs.next();

            let maybe_initial_value = if let Some(initial_value_or_properties) =
                &maybe_initial_value_or_properties
                && let Rule::init_value = initial_value_or_properties.as_rule()
            {
                let initial_value = initial_value_or_properties.to_string();
                maybe_initial_value_or_properties = pairs.next();
                Some(initial_value)
            } else {
                None
            };

            let maybe_properties = if let Some(initial_value_or_properties) =
                maybe_initial_value_or_properties
                && let Rule::argument_properties = initial_value_or_properties.as_rule()
            {
                Some(Self::parse_properties(initial_value_or_properties).unwrap())
            } else {
                None
            };

            let maybe_last_property =
                maybe_properties.and_then(|properties| properties.last().copied());

            if default_argument_property.is_out() {
                assert!(maybe_last_property.is_none_or(|last_property| { last_property.is_out() }));
            }

            let property = maybe_last_property.unwrap_or(default_argument_property);

            if property.is_out() {
                assert!(maybe_initial_value.is_none());
            }

            Some(Self {
                variable_type,
                identifier,
                maybe_initial_value,
                property,
            })
        } else {
            None
        }
    }

    pub fn parse_properties(pair: Pair<Rule>) -> Option<Vec<ArgumentProperty>> {
        if let Rule::argument_properties = pair.as_rule() {
            Some(
                pair.into_inner()
                    .map(|pair| ArgumentProperty::parse(pair).unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn identifier(&self) -> &Ident {
        &self.identifier
    }

    pub fn variable_type(&self) -> VariableType {
        self.variable_type
    }

    pub fn to_field_type_tokens(&self) -> TokenStream {
        let name_identifier = &self.identifier;
        let type_identifier = self.variable_type;

        quote! {
            pub #name_identifier: #type_identifier
        }
    }

    pub fn to_field_value_tokens(&self) -> TokenStream {
        let name_identifier = &self.identifier;
        let default_value = self.variable_type.default_value_tokens();

        quote! {
            #name_identifier: #default_value
        }
    }
}

impl StructFieldDefiner for Argument {
    fn identifier(&self) -> Ident {
        self.identifier().clone()
    }

    fn type_tokens(&self) -> impl ToTokens {
        self.variable_type()
    }
}

impl StructFieldInstantiator for Argument {
    fn identifier(&self) -> Ident {
        self.identifier().clone()
    }

    fn value_tokens(&self) -> impl ToTokens {
        self.variable_type().default_value_tokens()
    }
}

impl ArgumentDefiner for Argument {
    fn identifier(&self) -> Ident {
        self.identifier().clone()
    }

    fn type_tokens(&self) -> impl ToTokens {
        let type_tokens = self.variable_type();

        match self.property {
            ArgumentProperty::In => quote! { #type_tokens },
            ArgumentProperty::Out => quote! { *mut #type_tokens },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
pub enum ArgumentProperty {
    In,
    Out,
}

impl ArgumentProperty {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::argument_property = pair.as_rule() {
            match pair.to_string().as_str() {
                "#in" => Some(Self::In),
                "#out" => Some(Self::Out),
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum MethodElement {
    MethodVariable(MethodVariable),
    Verbatim(Verbatim),
}

impl MethodElement {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::method_variable => MethodVariable::parse(pair).map(Self::MethodVariable),
            Rule::verbatim => Verbatim::parse(pair).map(Self::Verbatim),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MethodVariable {
    pub r#type: VariableType,
    pub name: String,
    pub initial_value: Option<String>,
    pub properties: Option<Vec<ArgumentProperty>>,
}

impl MethodVariable {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::method_variable = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let r#type = VariableType::parse(pairs.next().unwrap()).unwrap();
            let name = pairs.next().unwrap().to_string();

            let mut initial_value_or_properties = pairs.next().unwrap();

            let initial_value = if let Rule::init_value = initial_value_or_properties.as_rule() {
                let initial_value = initial_value_or_properties.to_string();
                initial_value_or_properties = pairs.next().unwrap();
                Some(initial_value)
            } else {
                None
            };

            let properties =
                if let Rule::argument_properties = initial_value_or_properties.as_rule() {
                    Some(Self::parse_properties(initial_value_or_properties).unwrap())
                } else {
                    None
                };

            Some(Self {
                r#type,
                name,
                initial_value,
                properties,
            })
        } else {
            None
        }
    }

    pub fn parse_properties(pair: Pair<Rule>) -> Option<Vec<ArgumentProperty>> {
        if let Rule::argument_properties = pair.as_rule() {
            Some(
                pair.into_inner()
                    .map(|pair| ArgumentProperty::parse(pair).unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }
}
