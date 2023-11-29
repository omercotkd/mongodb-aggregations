extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use core::panic;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PipelineStage, attributes(pipeline_stage))]
pub fn macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let attributes = parse_struct_attributes(&input);

    let into_document = impl_into_document(&input, &attributes.name);

    let into_stage = impl_into_stage(&input, &attributes.name, &attributes.location);

    quote! {
        #into_document
        #into_stage
    }
    .into()
}

struct StageAttribute {
    name: String,
    location: String,
}

fn parse_struct_attributes(ast: &syn::DeriveInput) -> StageAttribute {
    const LOCATIONS: [&str; 3] = ["any", "first", "last"];

    let mut attributes = StageAttribute {
        name: ast.ident.to_string().to_camal_case(),
        location: String::from("any"),
    };

    if ast.attrs.is_empty() {
        return attributes;
    }

    ast.attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("pipeline_stage"))
        .for_each(|attribute| {
            match attribute.meta {
                syn::Meta::List(ref list) => {
                    
                },
                _ => panic!("Incorrect format for using the `pipeline_stage` attribute."),
            }
        });
    return attributes;
}

fn impl_into_stage(
    ast: &syn::DeriveInput,
    stage_name: &String,
    stage_location: &String,
) -> TokenStream2 {
    let name = &ast.ident;

    quote! {
        impl Into<crate::Stage> for #name {
            fn into(self) -> crate::Stage {
                crate::Stage::new(#stage_location.into(), self.into(), #stage_name)
            }
        }
    }
}

fn impl_into_document(ast: &syn::DeriveInput, stage_name: &String) -> TokenStream2 {
    let name = &ast.ident;

    quote! {
        impl Into<bson::Document> for #name {
            fn into(self) -> bson::Document {

                let mut document = bson::Document::new();
                // TODO camel case each struct field and insert into document
                document.insert(#stage_name, #stage_name);

                document
            }
        }
    }
}

trait ToCamalCase {
    fn to_camal_case(&self) -> String;
}

impl ToCamalCase for String {
    fn to_camal_case(&self) -> String {
        let mut result = String::from("");
        let mut make_upper = false;

        for c in self.chars() {
            match c {
                '_' => make_upper = true,
                _ if !make_upper && c.is_ascii_uppercase() => result.push(c.to_ascii_lowercase()),
                _ if make_upper => result.push(c.to_ascii_uppercase()),
                _ => result.push(c),
            }
        }

        result
    }
}
