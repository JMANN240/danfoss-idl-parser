use pest::iterators::Pair;
use proc_macro2::{Ident, TokenStream};
use quote::{IdentFragment, ToTokens, format_ident, quote};

use crate::{
    Rule,
    ast::{
        common::Verbatim,
        xclass::{method::Method, variable::Variable},
    },
};

pub mod method;
pub mod variable;

#[derive(Debug, Clone)]
pub struct XClass {
    pub identifier: Ident,
    pub properties: Option<Vec<XClassProperty>>,
    pub elements: Vec<XClassElement>,
}

impl XClass {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::xclass = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let identifier = format_ident!("{}", pairs.next().unwrap().to_string());

            let mut properties_or_elements = pairs.next().unwrap();

            let properties = if let Rule::xclass_properties = properties_or_elements.as_rule() {
                let properties = Self::parse_properties(properties_or_elements).unwrap();
                properties_or_elements = pairs.next().unwrap();
                Some(properties)
            } else {
                None
            };

            let elements = Self::parse_elements(properties_or_elements).unwrap();

            Some(Self {
                identifier,
                properties,
                elements,
            })
        } else {
            None
        }
    }

    pub fn parse_properties(pair: Pair<Rule>) -> Option<Vec<XClassProperty>> {
        if let Rule::xclass_properties = pair.as_rule() {
            Some(
                pair.into_inner()
                    .map(|pair| XClassProperty::parse(pair).unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn parse_elements(pair: Pair<Rule>) -> Option<Vec<XClassElement>> {
        if let Rule::xclass_elements = pair.as_rule() {
            Some(
                pair.into_inner()
                    .map(|pair| XClassElement::parse(pair).unwrap())
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn properties_of_name(
        &self,
        xlcass_property_name: XClassPropertyName,
    ) -> Option<impl Iterator<Item = &XClassProperty>> {
        self.properties.as_ref().map(move |xclass_properties| {
            xclass_properties
                .iter()
                .filter(move |xclass_property| xclass_property.name == xlcass_property_name)
        })
    }

    pub fn last_property_of_name(
        &self,
        xlcass_property_name: XClassPropertyName,
    ) -> Option<&XClassProperty> {
        self.properties_of_name(xlcass_property_name)
            .and_then(|xclass_properties_of_name| xclass_properties_of_name.last())
    }

    pub fn last_init_property(&self) -> Option<&XClassProperty> {
        self.last_property_of_name(XClassPropertyName::Init)
    }

    pub fn last_process_property(&self) -> Option<&XClassProperty> {
        self.last_property_of_name(XClassPropertyName::Process)
    }

    pub fn methods(&self) -> impl Iterator<Item = &Method> {
        self.elements.iter().filter_map(|xclass_element| {
            if let XClassElement::Method(method) = xclass_element {
                Some(method)
            } else {
                None
            }
        })
    }

    pub fn methods_with_out_arguments(&self) -> impl Iterator<Item = &Method> {
        self.methods().filter(|method| method.has_out_arguments())
    }

    pub fn variables(&self) -> impl Iterator<Item = &Variable> {
        self.elements.iter().filter_map(|xclass_element| {
            if let XClassElement::Variable(variable) = xclass_element {
                Some(variable)
            } else {
                None
            }
        })
    }

    pub fn shared_variables(&self) -> impl Iterator<Item = &Variable> {
        self.variables().filter(|variable| variable.is_shared())
    }

    pub fn instance_variables(&self) -> impl Iterator<Item = &Variable> {
        self.variables().filter(|variable| !variable.is_shared())
    }

    pub fn identifier(&self) -> Ident {
        format_ident!("{}", &self.identifier)
    }
}

#[derive(Debug, Clone)]
pub struct XClassProperty {
    name: XClassPropertyName,
    verbatim: Option<Verbatim>,
}

impl XClassProperty {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::xclass_property = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let name = XClassPropertyName::parse(pairs.next().unwrap()).unwrap();

            let verbatim = pairs.next().map(|pair| Verbatim::parse(pair).unwrap());

            Some(Self { name, verbatim })
        } else {
            None
        }
    }

    pub fn name(&self) -> XClassPropertyName {
        self.name
    }

    pub fn verbatim(&self) -> Option<&Verbatim> {
        self.verbatim.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XClassPropertyName {
    Init,
    Process,
}

impl XClassPropertyName {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::xclass_property_name = pair.as_rule() {
            match pair.to_string().as_str() {
                "#init" => Some(Self::Init),
                "#process" => Some(Self::Process),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl IdentFragment for XClassPropertyName {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Init => "INIT",
                Self::Process => "PROCESS",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum XClassElement {
    Method(Method),
    Variable(Variable),
    Verbatim(Verbatim),
}

impl XClassElement {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::method => Method::parse(pair).map(Self::Method),
            Rule::variable => Variable::parse(pair).map(Self::Variable),
            Rule::verbatim => Verbatim::parse(pair).map(Self::Verbatim),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IntegerType {
    U8,
    S8,
    U16,
    S16,
    U32,
    S32,
}

impl IntegerType {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::integer_type = pair.as_rule() {
            match pair.to_string().as_str() {
                "U8" => Some(Self::U8),
                "S8" => Some(Self::S8),
                "U16" => Some(Self::U16),
                "S16" => Some(Self::S16),
                "U32" => Some(Self::U32),
                "S32" => Some(Self::S32),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn default_value_tokens() -> TokenStream {
        quote! { 0 }
    }
}

impl ToTokens for IntegerType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::U8 => quote! { u8 },
            Self::S8 => quote! { i8 },
            Self::U16 => quote! { u16 },
            Self::S16 => quote! { i16 },
            Self::U32 => quote! { u32 },
            Self::S32 => quote! { i32 },
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VariableType {
    IntegerType(IntegerType),
    Bool,
}

impl VariableType {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::variable_type = pair.as_rule() {
            if let Some(pair) = pair.into_inner().next() {
                IntegerType::parse(pair).map(Self::IntegerType)
            } else {
                Some(Self::Bool)
            }
        } else {
            None
        }
    }

    pub fn default_value_tokens(&self) -> TokenStream {
        match self {
            Self::IntegerType(_) => IntegerType::default_value_tokens(),
            Self::Bool => quote! { false },
        }
    }
}

impl ToTokens for VariableType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::IntegerType(integer_type) => integer_type.to_tokens(tokens),
            Self::Bool => tokens.extend(quote! { bool }),
        };
    }
}
