extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemEnum};

#[proc_macro_derive(Fnum)]
pub fn derive_fnum(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemEnum);
    let enum_name = item.ident;
    let variants = item.variants;

    let variant_idx_arms = variants.iter().enumerate().map(|(i, variant)| {
        let ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(_) => {
                quote! {
                    #enum_name::#ident{..} => {#i}
                }
            }
            syn::Fields::Unnamed(fields) => {
                let fs = fields.unnamed.iter().map(|_| {
                    quote! {_}
                }).collect::<Vec<_>>();
                quote! {
                    #enum_name::#ident(#(#fs),*) => {#i}
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #enum_name::#ident => {#i}
                }
            }
        }
    }).collect::<Vec<_>>();

    let size_of_variant_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(fields) => {
                let field_idents = fields.named.iter().map(|field| field.ident.clone().unwrap()).collect::<Vec<_>>();
                let pointers = field_idents.iter().map(|i| quote! {right_pointer(#i)}).collect::<Vec<_>>();
                quote! {
                    #enum_name::#ident{#(#field_idents),*} => {[#(#pointers),*].iter().max().unwrap() - pointer(&e)}
                }
            }
            syn::Fields::Unnamed(fields) => {
                let field_idents = fields.unnamed.iter().enumerate().map(|(i, _)| {
                    quote::format_ident!("field{}", i)
                }).collect::<Vec<_>>();
                let pointers = field_idents.iter().map(|i| quote! {right_pointer(#i)}).collect::<Vec<_>>();
                quote! {
                    #enum_name::#ident(#(#field_idents),*) => {[#(#pointers),*].iter().max().unwrap() - pointer(&e)}
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #enum_name::#ident => {2} // dame kamo
                }
            }
        }
    }).collect::<Vec<_>>();


    let uninit_variant_arms = variants.iter().enumerate().map(|(i, variant)| {
        let ident = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(fields) => {
                let inits = fields.named.iter().map(|field| {
                    let name = field.ident.clone().unwrap(); // unwrap daijoubu?
                    quote! {
                        #name: std::mem::MaybeUninit::uninit().assume_init()
                    }
                }).collect::<Vec<_>>();
                quote! {
                    #i => #enum_name::#ident{#(#inits),*}
                }
            }
            syn::Fields::Unnamed(fields) => {
                let inits = fields.unnamed.iter().map(|_| quote! { std::mem::MaybeUninit::uninit().assume_init() }).collect::<Vec<_>>();
                quote! {
                    #i => #enum_name::#ident(#(#inits),*)
                }
            }
            syn::Fields::Unit => {
                quote! {
                    #i => #enum_name::#ident
                }
            }
        }
    }).collect::<Vec<_>>();

    let variant_num = variants.len();
    let gen = quote! {
        impl fnum::Fnum for #enum_name {
            fn variant_count() -> usize {
                #variant_num
            }
            fn variant_idx(&self) -> usize {
                match self {
                    #(#variant_idx_arms),*
                }
            }
            unsafe fn uninit_variant(idx: usize) -> Self {
                assert!(idx < Self::variant_count());
                match idx {
                    #(#uninit_variant_arms,)*
                    _ => unreachable!(),
                }
            }
            fn size_of_variant(idx: usize) -> usize {
                fn pointer<T>(t: &T) -> usize {
                    t as *const _ as usize
                }
                fn right_pointer<T>(t: &T) -> usize {
                    unsafe {(t as *const T).offset(1) as usize}
                }
                let e = unsafe { Self::uninit_variant(idx) };
                let size = match &e {
                    #(#size_of_variant_arms),*
                };
                std::mem::forget(e);
                size
            }
        }
    };
    gen.into()
}
