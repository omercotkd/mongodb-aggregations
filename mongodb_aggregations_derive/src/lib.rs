extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(MyMacroHere)]
pub fn my_macro_here_derive(input: TokenStream) -> TokenStream { 
    todo!()
}
