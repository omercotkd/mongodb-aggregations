extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod helpers;
mod options;
use helpers::ToCamalCase;
use options::Options;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PipelineStage, attributes(pipeline_stage))]
pub fn macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Parsing the options aka the attributes of the struct
    let opts = Options::parse(&input);

    // Generating the impl of Into<Document> if the `impl_into_document` option is set
    let into_document = match opts.impl_into_document {
        true => impl_into_stage_document(&input, &opts),
        false => TokenStream2::new(),
    };

    let into_stage = impl_into_stage(&input, &opts);

    quote! {
        #into_document
        #into_stage
    }
    .into()
}

fn impl_into_stage(ast: &syn::DeriveInput, opts: &Options) -> TokenStream2 {
    let name = &ast.ident;

    let stage_name = &opts.name;

    let stage_location = &opts.location;

    if opts.internal_impl {
        return quote! {
            impl Into<crate::Stage> for #name {
                fn into(self) -> crate::Stage {
                    crate::Stage::new(#stage_location.into(), self.into(), #stage_name)
                }
            }
        };
    }

    quote! {
        impl Into<::mongodb_aggregations::Stage> for #name {
            fn into(self) -> ::mongodb_aggregations::Stage {
                ::mongodb_aggregations::Stage::new(#stage_location.into(), self.into(), #stage_name)
            }
        }
    }
}

fn impl_into_stage_document(ast: &syn::DeriveInput, opts: &Options) -> TokenStream2 {
    let name = &ast.ident;

    // If the `document_field` option is set, we will use the value of this field as the document
    if let Some(document_field) = &opts.document_field {
        let stage_name = &opts.name;

        return quote! {
            impl Into<::bson::Document> for #name {
                fn into(self) -> ::bson::Document {
                    let mut stage_doc = ::bson::Document::new();

                    stage_doc.insert(#stage_name, self.#document_field);

                    stage_doc
                }
            }
        };
    }

    let struct_as_document = struct_into_document(ast);

    let stage_name = &opts.name;

    quote! {
        impl Into<::bson::Document> for #name {
            fn into(self) -> ::bson::Document {
                // this will give us a var named `document` of type `::bson::Document`
                #struct_as_document

                let mut stage_doc = ::bson::Document::new();

                stage_doc.insert(#stage_name, document);

                stage_doc
            }
        }
    }
}

fn struct_into_document(ast: &syn::DeriveInput) -> TokenStream2 {
    // Get the fields of the struct
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
            // Getting the field attributes, keeping only the `cfg` ones
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

        // Generating the document insert for the field
        let insert_quote = quote! {
            #(#field_attrs)*
            document.insert(#key, self.#field_name);
        };

        // if the field is an Option, we need to check if it is Some before inserting it
        // This is not a serilization of the struct so this behavior is intentional
        match field_type {
            syn::Type::Path(ref type_path) => {
                // getting the type name
                let type_name = type_path.path.segments.last().unwrap().ident.to_string();

                if type_name == "Option" {
                    document_fields.push(quote! {
                        #(#field_attrs)*
                        if let Some(value) = self.#field_name {
                            document.insert(#key, value);
                        }
                    });
                } else {
                    document_fields.push(insert_quote)
                }
            }
            _ => document_fields.push(insert_quote),
        }
    }

    quote! {
        let mut document = ::bson::Document::new();
        // Inserting the fields into the inner document
        #(#document_fields)*
    }
}
