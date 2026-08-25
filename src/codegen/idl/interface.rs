use std::convert::identity;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::{
    ast::{Interface, InterfaceElement, xclass::XClass},
    codegen::{
        idl::{
            instance_struct::InstanceStruct,
            method::{
                XClassMethodDefinition, XClassPropertyMethodDefinition,
                XClassVariableGetMethodDefinition, XClassVariableSetMethodDefinition,
            },
            shared_struct::SharedStruct,
        },
        rust::{
            definition::{MethodDefiner, MethodDefinition, StructDefiner},
            instantiation::StaticAssigner,
        },
    },
};

pub struct InterfaceDefinition {
    idl_checksum: u64,
    interface: Interface,
}

impl InterfaceDefinition {
    pub fn new(idl_checksum: u64, interface: Interface) -> Self {
        Self {
            idl_checksum,
            interface,
        }
    }

    pub fn idl_checksum(&self) -> u64 {
        self.idl_checksum
    }

    pub fn interface(&self) -> &Interface {
        &self.interface
    }
}

impl ToTokens for InterfaceDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let interface_element_definitions = self
            .interface()
            .interface_elements()
            .iter()
            .cloned()
            .map(|interface_element| {
                InterfaceElementDefinition::new(self.idl_checksum(), interface_element)
            });

        tokens.extend(quote! {
            #![no_std]

            use core::panic::PanicInfo;

            #[panic_handler]
            fn panic(_: &PanicInfo) -> ! {
                loop {}
            }

            #(#interface_element_definitions)*
        });
    }
}

#[derive(Debug, Clone)]
pub struct InterfaceElementDefinition {
    idl_checksum: u64,
    interface_element: InterfaceElement,
}

impl InterfaceElementDefinition {
    pub fn new(idl_checksum: u64, interface_element: InterfaceElement) -> Self {
        Self {
            idl_checksum,
            interface_element,
        }
    }

    pub fn idl_checksum(&self) -> u64 {
        self.idl_checksum
    }

    pub fn interface_element(&self) -> &InterfaceElement {
        &self.interface_element
    }
}

impl ToTokens for InterfaceElementDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.interface_element() {
            InterfaceElement::XClass(xclass) => {
                XClassDefinition::new(self.idl_checksum(), xclass.clone()).to_tokens(tokens)
            }
            InterfaceElement::Insert(insert) => insert.to_tokens(tokens),
        }
    }
}

pub struct XClassDefinition {
    idl_checksum: u64,
    xclass: XClass,
}

impl XClassDefinition {
    pub fn new(idl_checksum: u64, xclass: XClass) -> Self {
        Self {
            idl_checksum,
            xclass,
        }
    }

    pub fn idl_checksum(&self) -> u64 {
        self.idl_checksum
    }

    pub fn xclass(&self) -> &XClass {
        &self.xclass
    }

    pub fn property_method_definitions(&self) -> impl Iterator<Item = MethodDefinition> {
        [
            self.xclass().last_init_property(),
            self.xclass().last_process_property(),
        ]
        .into_iter()
        .filter_map(|maybe_xclass_property| {
            maybe_xclass_property.map(|xclass_property| {
                XClassPropertyMethodDefinition::new(self.xclass().clone(), xclass_property.clone())
                    .to_method_definition()
            })
        })
    }

    pub fn method_definitions(&self) -> impl Iterator<Item = MethodDefinition> {
        self.xclass()
            .methods()
            .cloned()
            .map(|method| XClassMethodDefinition::new(self.xclass().clone(), method))
            .map(|xclass_method_definition| xclass_method_definition.to_method_definition())
    }

    pub fn variable_method_definitions(&self) -> impl Iterator<Item = MethodDefinition> {
        self.xclass().variables().flat_map(|xclass_variable| {
            [
                xclass_variable.is_get().then(|| {
                    XClassVariableGetMethodDefinition::new(
                        self.xclass().clone(),
                        xclass_variable.clone(),
                    )
                    .to_method_definition()
                }),
                xclass_variable.is_set().then(|| {
                    XClassVariableSetMethodDefinition::new(
                        self.xclass().clone(),
                        xclass_variable.clone(),
                    )
                    .to_method_definition()
                }),
            ]
            .into_iter()
            .filter_map(identity)
        })
    }
}

impl ToTokens for XClassDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let shared_struct = SharedStruct::new(self.idl_checksum(), self.xclass().clone());

        let out_arguments_struct_definitions = self
            .xclass()
            .methods_with_out_arguments()
            .map(StructDefiner::to_struct_definition);

        let shared_struct_definition = shared_struct.to_struct_definition();
        let shared_struct_static_assignment = shared_struct.to_static_assignment();

        let instance_struct_definition =
            InstanceStruct::new(self.idl_checksum(), self.xclass().clone()).to_struct_definition();

        let xclass_property_method_definitions = self.property_method_definitions();

        let xclass_variable_method_definitions = self.variable_method_definitions();

        let xclass_method_definitions = self.method_definitions();

        tokens.extend(quote! {
            #shared_struct_definition

            #(#out_arguments_struct_definitions)*

            #shared_struct_static_assignment

            #instance_struct_definition

            #(#xclass_property_method_definitions)*

            #(#xclass_variable_method_definitions)*

            #(#xclass_method_definitions)*
        });
    }
}
