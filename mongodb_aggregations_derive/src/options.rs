use crate::helpers::ToCamalCase;
use darling::FromDeriveInput;

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(pipeline_stage))]
pub(crate) struct Options {
    #[darling(default = "String::new")]
    pub(crate) name: String,
    #[darling(default = "Options::default_location")]
    pub(crate) location: String,
    #[darling(default = "Options::default_impl_into_document")]
    pub(crate) impl_into_document: bool,
}

impl Options {
    pub(crate) fn parse(ast: &syn::DeriveInput) -> Options {
        let mut opts = Options::from_derive_input(ast).expect("Failed to parse attributes");

        if opts.name.is_empty() {
            opts.name = ast.ident.to_string().to_camal_case();
        }

        if !Options::locations().contains(&opts.location.as_str()) {
            panic!("Incorrect location for using the `pipeline_stage` attribute.");
        }

        if !opts.name.starts_with('$') {
            opts.name = format!("${}", opts.name);
        }

        return opts;
    }

    fn default_location() -> String {
        String::from("any")
    }

    fn default_impl_into_document() -> bool {
        false
    }

    fn locations() -> &'static [&'static str] {
        &["any", "first", "last"]
    }
}
