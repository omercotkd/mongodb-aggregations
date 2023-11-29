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

    ast.attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("pipeline_stage"))
        .for_each(|attribute| match attribute.meta {
            syn::Meta::List(ref list) => {}
            _ => panic!("Incorrect format for using the `pipeline_stage` attribute."),
        });

    if !LOCATIONS.contains(&attributes.location.as_str()) {
        panic!("Incorrect location for using the `pipeline_stage` attribute.");
    }

    if !attributes.name.starts_with('$') {
        attributes.name = format!("${}", attributes.name);
    }

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

    let fields = match ast.data {
        syn::Data::Struct(ref data_struct) => match data_struct.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            syn::Fields::Unnamed(_) => panic!("Unnamed fields are not supported"),
            syn::Fields::Unit => panic!("Unit fields are not supported"),
        },
        syn::Data::Enum(_) => panic!("Enums are not supported"),
        syn::Data::Union(_) => panic!("Unions are not supported"),
    }
    .iter()
    .map(|field| {
        (
            field.ident.as_ref().unwrap(),
            &field.ty,
            field
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("cfg"))
                .collect::<Vec<_>>(),
        )
    })
    .collect::<Vec<_>>();

    let mut document_fields = Vec::new();
    
    for (field_name, field_type, field_attrs) in fields {
        let key = field_name.to_string().to_camal_case();

        let normal_quote = quote! {
            #(#field_attrs)*
            inner_document.insert(#key, self.#field_name);
        };

        match field_type {
            syn::Type::Path(ref type_path) => {
                let type_name = type_path.path.segments.last().unwrap().ident.to_string();

                if type_name == "Option" {
                    document_fields.push(quote! {
                        #(#field_attrs)*
                        if let Some(value) = self.#field_name {
                            inner_document.insert(#key, value);
                        }
                    });
                } else {
                    document_fields.push(normal_quote)
                }
            }
            _ => document_fields.push(normal_quote),
        }
    }

    quote! {
        impl Into<::bson::Document> for #name {
            fn into(self) -> ::bson::Document {

                let mut inner_document = ::bson::Document::new();

                #(#document_fields)*

                let mut document = ::bson::Document::new();

                document.insert(#stage_name, inner_document);

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
        let mut first = true;

        for c in self.chars() {
            match c {
                '_' => make_upper = true,
                _ if first => {
                    result.push(c.to_ascii_lowercase());
                    first = false;
                }
                _ if make_upper => {
                    result.push(c.to_ascii_uppercase());
                    make_upper = false;
                }
                _ => result.push(c),
            }
        }

        result
    }
}
