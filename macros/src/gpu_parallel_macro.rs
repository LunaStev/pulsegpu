use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, Block, parse::{Parse, ParseStream}, Token, LitInt, braced, Result};

pub struct GpuParallelInput {
    pub threads: u32,
    pub block_size: Option<u32>,
    pub ident: Ident,
    pub body: Box<Block>,
}

impl Parse for GpuParallelInput {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Ident>()?; // threads
        input.parse::<Token![=]>()?;
        let threads_lit: LitInt = input.parse()?;
        let threads = threads_lit.base10_parse::<u32>()?;

        let block_size = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            input.parse::<Ident>()?; // block
            input.parse::<Token![=]>()?;
            let block_lit: LitInt = input.parse()?;
            Some(block_lit.base10_parse::<u32>()?)
        } else {
            None
        };

        input.parse::<Token![,]>()?;
        let ident: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let body: Block = content.parse()?;

        Ok(GpuParallelInput {
            threads,
            block_size,
            ident,
            body: Box::new(body),
        })
    }
}

pub fn gpu_parallel(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as GpuParallelInput);
    let threads = parsed.threads;
    let block_size = parsed.block_size.unwrap_or(0);
    let ident = parsed.ident;
    let body = parsed.body;

    let expanded = quote! {
        {
            ::std::println!("🚀 gpu_parallel! launched with {} threads, block size: {}", #threads, #block_size);
            for #ident in 0..#threads {
                {
                    #(#body)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
