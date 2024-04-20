use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(RecursivePrimeFactorization)]
pub fn recursive_prime_factorization_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_recursive_prime_factorization(&ast)
}

fn impl_recursive_prime_factorization(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl crate::traits::PrimeFactorization for #name {
            fn prime_factorization(n: &bnum::types::U256) -> Vec<bnum::types::U256> {
                crate::orchestration::RecursivePrimeFactorization::<Self, crate::primality_test::MillerRabin>::prime_factorization(n)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(FactorizationCommand)]
pub fn factorization_command_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_factorization_command(&ast)
}

fn impl_factorization_command(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl crate::traits::Command for #name {
            fn run(&self, n: &U256) -> String {
                crate::Factorization::new(n, Self::prime_factorization(n)).to_string()
            }
        }
        impl crate::traits::FactorizationCommand for # name {}
    };
    gen.into()
}
