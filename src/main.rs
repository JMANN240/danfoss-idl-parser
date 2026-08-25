use std::fs;

use idl_parser::{IDLParser, Rule, ast::Interface, codegen::idl::interface::InterfaceDefinition};
use pest::Parser;
use quote::ToTokens;
use syn::File;

fn main() {
    let data = fs::read("Adder.idl").unwrap();
    let checksum = cargo_ccp::idl_checksum(&data);

    let string = String::from_utf8(data).unwrap();

    let mut pairs = IDLParser::parse(Rule::interface, &string).unwrap();

    if let Some(interface) = Interface::parse(pairs.next().unwrap()) {
        let interface_definition = InterfaceDefinition::new(checksum, interface);

        let parsed = syn::parse2::<File>(interface_definition.to_token_stream()).unwrap();

        println!("{}", prettyplease::unparse(&parsed));
    }
}
