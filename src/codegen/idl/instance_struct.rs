use quote::{format_ident, quote};
use syn::Ident;

use crate::{
    ast::xclass::XClass,
    codegen::{
        idl::shared_struct::SharedStruct,
        rust::definition::{StructDefiner, StructFieldDefiner, StructFieldDefinition},
    },
};

pub struct InstanceStruct {
    idl_checksum: u64,
    xclass: XClass,
}

impl InstanceStruct {
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

    pub fn shared_field_identifier() -> Ident {
        format_ident!("shared")
    }
}

impl StructDefiner for InstanceStruct {
    fn identifier(&self) -> Ident {
        self.xclass().identifier().clone()
    }

    fn fields(&self) -> Vec<StructFieldDefinition> {
        let shared_struct = SharedStruct::new(self.idl_checksum(), self.xclass().clone());

        let shared_struct_type_identifier = shared_struct.type_identifier();

        [
            StructFieldDefinition::new(format_ident!("reserved"), quote! { u32 }),
            StructFieldDefinition::new(
                Self::shared_field_identifier(),
                quote! { *mut #shared_struct_type_identifier },
            ),
        ]
        .into_iter()
        .chain(
            self.xclass()
                .instance_variables()
                .map(StructFieldDefiner::to_struct_field_definition),
        )
        .collect::<Vec<_>>()
    }
}
