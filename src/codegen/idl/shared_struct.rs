use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::Ident;

use crate::{
    ast::xclass::XClass,
    codegen::rust::{
        definition::{StructDefiner, StructFieldDefiner, StructFieldDefinition},
        instantiation::{
            StaticAssigner, StructFieldInstantiation, StructFieldInstantiator, StructInstantiator,
        },
    },
};

pub struct SharedStruct {
    idl_checksum: u64,
    xclass: XClass,
}

impl SharedStruct {
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

    pub fn init_flag_field_identifier() -> Ident {
        format_ident!("init_flag")
    }

    pub fn type_identifier(&self) -> Ident {
        format_ident!(
            "{}_{:X}_SHARED_TYPE",
            self.xclass().identifier(),
            self.idl_checksum()
        )
    }

    pub fn instance_identifier(&self) -> Ident {
        format_ident!(
            "{}_{:X}_SHARED_INSTANCE",
            self.xclass().identifier(),
            self.idl_checksum()
        )
    }

    pub fn struct_field_definitions(&self) -> impl Iterator<Item = StructFieldDefinition> {
        [StructFieldDefinition::new(
            Self::init_flag_field_identifier(),
            quote! { u32 },
        )]
        .into_iter()
        .chain(
            self.xclass()
                .shared_variables()
                .map(StructFieldDefiner::to_struct_field_definition),
        )
        .chain(
            self.xclass()
                .methods_with_out_arguments()
                .map(StructFieldDefiner::to_struct_field_definition),
        )
    }
}

impl StructDefiner for SharedStruct {
    fn identifier(&self) -> Ident {
        self.type_identifier()
    }

    fn field_tokens(&self) -> TokenStream {
        let struct_field_definitions = self.struct_field_definitions().collect::<Vec<_>>();

        quote! {
            #(#struct_field_definitions),*
        }
    }
}

impl StructInstantiator for SharedStruct {
    fn identifier(&self) -> Ident {
        self.type_identifier()
    }

    fn fields(&self) -> Vec<StructFieldInstantiation> {
        let shared_variable_field_instantiations = self
            .xclass()
            .shared_variables()
            .map(StructFieldInstantiator::to_struct_field_instantiation);

        let out_argument_field_instantiations = self
            .xclass()
            .methods_with_out_arguments()
            .map(StructFieldInstantiator::to_struct_field_instantiation);

        [StructFieldInstantiation::new(
            Self::init_flag_field_identifier(),
            quote! { 0 },
        )]
        .into_iter()
        .chain(shared_variable_field_instantiations)
        .chain(out_argument_field_instantiations)
        .collect()
    }
}

impl StaticAssigner for SharedStruct {
    fn identifier(&self) -> Ident {
        self.instance_identifier()
    }

    fn type_tokens(&self) -> impl ToTokens {
        self.type_identifier()
    }

    fn value_tokens(&self) -> impl ToTokens {
        self.to_struct_instantiation()
    }
}
