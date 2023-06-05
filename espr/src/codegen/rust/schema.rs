use crate::ir::*;

use check_keyword::CheckKeyword;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

// HACK: fix special cases properly.
pub fn make_name(ident: &str) -> String {
    let mut name = ident.to_screaming_snake_case();
    match name.as_str() {
        "AXIS_1_PLACEMENT" => name = "AXIS1_PLACEMENT".to_string(),
        "AXIS_2_PLACEMENT_2D" => name = "AXIS2_PLACEMENT_2D".to_string(),
        "AXIS_2_PLACEMENT_3D" => name = "AXIS2_PLACEMENT_3D".to_string(),
        _ => {}
    };
    name
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CratePrefix {
    Internal,
    External,
}

impl CratePrefix {
    pub fn as_path(&self) -> syn::Path {
        match self {
            CratePrefix::Internal => syn::parse_str("crate").unwrap(),
            CratePrefix::External => syn::parse_str("::ruststep").unwrap(),
        }
    }
}

impl IR {
    pub fn to_token_stream(&self, prefix: CratePrefix) -> TokenStream {
        let schemas: Vec<_> = self
            .schemas
            .iter()
            .map(|schema| schema.to_token_stream(prefix))
            .collect();
        quote! { #(#schemas)* }
    }
}

struct PartialEntityMapping {
    pub name: String,
    pub attributes: Vec<String>,
}

impl ToTokens for PartialEntityMapping {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let PartialEntityMapping { name, attributes } = self;
        tokens.append_all(quote! {
            #name => &[ #(#attributes,)* ],
        });
    }
}

impl Schema {
    pub fn to_token_stream(&self, prefix: CratePrefix) -> TokenStream {
        let name = format_ident!("{}", self.name);
        let types = &self.types;
        let entities = &self.entities;
        let type_decls = self.types.iter().filter(|e| match e {
            TypeDecl::Enumeration(_) => false,
            _ => true,
        });

        let mut partials = Vec::new();
        for entity in entities {
            let mut attributes = Vec::new();
            for attribute in &entity.attributes {
                attributes.push(attribute.name.clone());
            }
            partials.push(PartialEntityMapping {
                name: make_name(&entity.name),
                attributes,
            });
        }

        let expanded_entities: Vec<Entity> = entities.iter().map(|e| e.expand(entities)).collect();

        let mut complete = Vec::new();
        for ee in &expanded_entities {
            let mut attributes = Vec::new();
            for attribute in &ee.attributes {
                attributes.push(attribute.name.clone());
            }
            complete.push(PartialEntityMapping {
                name: make_name(&ee.name),
                attributes,
            });
        }

        let entity_types: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name.to_pascal_case()))
            .chain(
                type_decls
                    .clone()
                    .map(|e| format_ident!("{}", e.id().to_pascal_case())),
            )
            .collect();
        let holder_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}", e.name.as_str().into_safe()))
            .chain(
                type_decls
                    .clone()
                    .map(|e| format_ident!("{}", e.id().into_safe())),
            )
            .collect();
        let holders_name: Vec<_> = entities
            .iter()
            .map(|e| format_ident!("{}_holders", e.name))
            .chain(type_decls.map(|e| format_ident!("{}_holders", e.id())))
            .collect();

        let ruststep_path = prefix.as_path();

        quote! {
            pub mod #name {
                use #ruststep_path::{as_holder, Holder, TableInit, primitive::*, derive_more::*};
                use std::collections::HashMap;

                static COMPLETE: ::phf::Map<&'static str, &'static [&'static str]> = ::phf::phf_map! {
                    #( #complete )*
                };

                static PARTIALS: ::phf::Map<&'static str, &'static [&'static str]> = ::phf::phf_map! {
                    #( #partials )*
                };

                #[derive(Debug, Clone, PartialEq, Default, TableInit)]
                pub struct Tables {
                    #(
                    #holder_name: HashMap<u64, as_holder!(#entity_types)>,
                    )*
                }

                impl Tables {
                    #(
                    pub fn #holders_name(&self) -> &HashMap<u64, as_holder!(#entity_types)> {
                        &self.#holder_name
                    }
                    )*

                    pub fn complete_mappings(&self) -> &::phf::Map<&'static str, &'static [&'static str]> {
                        &COMPLETE
                    }

                    pub fn partial_mappings(&self) -> &::phf::Map<&'static str, &'static [&'static str]> {
                        &PARTIALS
                    }
                }

                #(#types)*
                #(#expanded_entities)*
            }
        }
    }
}
