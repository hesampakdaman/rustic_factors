use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(RecursivePrimeFactorization)]
pub fn recursive_prime_factorization_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_recursive_prime_factorization(&ast)
}

fn impl_recursive_prime_factorization(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl crate::traits::PrimeFactorization for #name {
            fn prime_factorization(n: &bnum::types::U512) -> Vec<bnum::types::U512> {
                crate::orchestration::FactorizeRecursiveWith::<Self, crate::primality_test::MillerRabin>::prime_factorization(n)
            }
        }
    };
    gen.into()
}
