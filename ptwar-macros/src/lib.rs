use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, ItemStruct};

extern crate proc_macro;

#[proc_macro_derive(Event)]
pub fn event(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    quote! {
        impl crate::event::Event for #name {
            fn get_name_static() -> &'static str {
                stringify!(#name)
            }

            fn get_name(&self) -> &'static str {
                stringify!(#name)
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn tick_system(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let name = &input.sig.ident;

    let first_arg = match input.sig.inputs.first().unwrap() {
        syn::FnArg::Typed(pat) => match pat.pat.as_ref() {
            syn::Pat::Ident(ident) => ident.ident.clone(),
            _ => panic!("First argument must be a variable"),
        },
        _ => panic!("First argument must be a typed argument"),
    };

    let second_arg = match input.sig.inputs.iter().nth(1).unwrap() {
        syn::FnArg::Typed(pat) => match pat.pat.as_ref() {
            syn::Pat::Ident(ident) => ident.ident.clone(),
            _ => panic!("Second argument must be a variable"),
        },
        _ => panic!("Second argument must be a typed argument"),
    };

    let block = input.block;

    quote! {
        pub struct #name{}

        #[async_trait]
        impl ::ptwar::worker::TickHandler for #name {
            async fn handle(&self, #first_arg: Tick, #second_arg: Arc<PtWarServer>) {
                #block
            }
        }
    }
    .into()
}
