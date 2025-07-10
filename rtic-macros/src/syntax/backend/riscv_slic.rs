use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    Result,
};

#[derive(Debug)]
pub struct BackendArgs(pub Option<TokenStream>);

impl Parse for BackendArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        match input.is_empty() {
            true => Ok(BackendArgs(None)),
            false => Ok(BackendArgs(Some(input.parse()?))),
        }
    }
}
