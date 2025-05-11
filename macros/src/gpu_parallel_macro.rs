use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, parse::{Parse, ParseStream}, Token, LitInt, braced, Result, Stmt};

pub struct GpuParallelInput {
    pub threads: u32,
    pub block_size: Option<u32>,
    pub ident: Ident,
    pub body: Vec<Stmt>,
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
        let body: Vec<Stmt> = content.call(syn::Block::parse_within)?;

        Ok(GpuParallelInput {
            threads,
            block_size,
            ident,
            body: *Box::new(body),
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
            let ir = pulsegpu_ir::parallel_ir::GpuParallelIR {
                threads: #threads,
                block_size: #block_size,
                body: stringify! {
                    #(#body)*
                }.to_string(),
            };
            pulsegpu_runtime::dispatcher::dispatch(ir);
        }
    };

    TokenStream::from(expanded)
}
