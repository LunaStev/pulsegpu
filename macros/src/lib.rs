mod gpu_parallel_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn gpu_parallel(input: TokenStream) -> TokenStream {
    gpu_parallel_macro::gpu_parallel(input)
}
