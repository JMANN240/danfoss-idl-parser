use pest::iterators::Pair;
use quote::{IdentFragment, ToTokens, format_ident};
use strum::EnumIs;
use syn::Ident;

use crate::{
    Rule,
    ast::xclass::VariableType,
    codegen::rust::{
        definition::StructFieldDefiner,
        instantiation::StructFieldInstantiator,
    },
};

#[derive(Debug, Clone)]
pub struct Variable {
    pub variable_type: VariableType,
    pub identifier: Ident,
    pub maybe_initial_value: Option<String>,
    pub maybe_properties: Option<Vec<VariableProperty>>,
}

impl Variable {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::variable = pair.as_rule() {
            let mut pairs = pair.into_inner();

            let variable_type = VariableType::parse(pairs.next().unwrap()).unwrap();
            let identifier = format_ident!("{}", pairs.next().unwrap().to_string());

            let mut variable = Self {
                variable_type,
                identifier,
                maybe_initial_value: None,
                maybe_properties: None,
            };

            if let Some(initial_value_or_properties) = pairs.next() {
                match initial_value_or_properties.as_rule() {
                    Rule::init_value => {
                        let initial_value = initial_value_or_properties.to_string();
                        variable.maybe_initial_value = Some(initial_value);

                        if let Some(properties) = pairs.next() {
                            variable.maybe_properties =
                                Some(Self::parse_properties(properties).unwrap());
                        }
                    }
                    Rule::variable_properties => {
                        variable.maybe_properties =
                            Some(Self::parse_properties(initial_value_or_properties).unwrap());
                    }
                    _ => unreachable!(),
                }
            }

            Some(variable)
        } else {
            None
        }
    }

    pub fn parse_properties(pair: Pair<Rule>) -> Option<Vec<VariableProperty>> {
        if let Rule::variable_properties = pair.as_rule() {
            Some(
                pair.into_inner()
                    .map(|pair| VariableProperty::parse(pair).unwrap())
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

    pub fn has_property(&self, variable_property: VariableProperty) -> bool {
        self.maybe_properties
            .as_ref()
            .is_some_and(|properties| properties.contains(&variable_property))
    }

    pub fn is_shared(&self) -> bool {
        self.has_property(VariableProperty::Shared)
    }

    pub fn is_get(&self) -> bool {
        self.has_property(VariableProperty::Get)
    }

    pub fn is_set(&self) -> bool {
        self.has_property(VariableProperty::Set)
    }
}

impl StructFieldDefiner for Variable {
    fn identifier(&self) -> Ident {
        self.identifier().clone()
    }

    fn type_tokens(&self) -> impl ToTokens {
        self.variable_type()
    }
}

impl StructFieldInstantiator for Variable {
    fn identifier(&self) -> Ident {
        self.identifier().clone()
    }

    fn value_tokens(&self) -> impl ToTokens {
        self.variable_type().default_value_tokens()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIs)]
pub enum VariableProperty {
    Shared,
    Get,
    Set,
}

impl VariableProperty {
    pub fn parse(pair: Pair<Rule>) -> Option<Self> {
        if let Rule::variable_property = pair.as_rule() {
            match pair.to_string().as_str() {
                "#shared" => Some(Self::Shared),
                "#get" => Some(Self::Get),
                "#set" => Some(Self::Set),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl IdentFragment for VariableProperty {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", match self {
            Self::Get => "GET",
            Self::Set => "SET",
            Self::Shared => unreachable!(),
        })
    }
}
