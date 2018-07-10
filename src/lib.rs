#![feature(proc_macro)]

#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;
use std::borrow::Borrow;
use std::iter::FromIterator;

fn impl_delegable_impl(impl_item : &syn::ItemImpl) -> quote::Tokens {
    let btype = impl_item.self_ty.borrow();
    if let syn::Type::Verbatim(ty) = btype {
        let name : syn::Ident = syn::parse2(ty.tts.clone()).expect("Expected identifier.");
        let mut methods = quote::Tokens::new();
        for item in &impl_item.items {
            if let syn::ImplItem::Method(ref item_method) = item {
                methods.append_all(add_method(&name, &item_method.sig));
            }
        }
        quote_delegable(&name, &methods)
    } else {
        panic!("Expected direct type in type position.");
    }
}

fn quote_delegable(name: &syn::Ident, methods: &quote::Tokens) -> quote::Tokens {
    quote! {
        pub mod delegate {
            pub trait #name {
                type Inner : super::#name;
                fn inner(&self) -> &Self::Inner;

                fn inner_mut(&mut self) -> &mut Self::Inner;

                fn into_inner(self) -> Self::Inner;

                fn from_inner(delegate : Self::Inner) -> Self;
            }

            impl<Proxy : #name> super::#name for Proxy {
                #methods
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum FirstArg {
    SelfValue,
    MutSelfValue,
    SelfRef,
    MutSelfRef,
    NotSelf
}

fn arg_self_kind(arg : Option<&syn::FnArg>) -> FirstArg {
    match arg {
        Some(fn_arg) => {
            match fn_arg {
                syn::FnArg::SelfRef(arg) => {
                    match arg.mutability {
                        Some(_) => FirstArg::MutSelfRef,
                        None => FirstArg::SelfRef
                    }
                },
                syn::FnArg::SelfValue(arg) => {
                    match arg.mutability {
                        Some(_) => FirstArg::MutSelfValue,
                        None => FirstArg::SelfValue
                    }
                },
                _ => {
                    FirstArg::NotSelf
                }  
            }
        },
        None => FirstArg::NotSelf
    }
}

fn get_call_args(first_arg : FirstArg, args : &syn::punctuated::Punctuated<syn::FnArg, Token!(,)>) 
-> syn::punctuated::Punctuated<syn::Ident, &Token!(,)> {
    let it = args.pairs();
    let it = if first_arg != FirstArg::NotSelf {
        // skip "self" in the call
        it.skip(1)
    } else {
        it.skip(0)
    };

    let it = it.map(|pair| {
        use syn::punctuated::Pair;
        let to_ident = |ref arg: &syn::FnArg| { 
            match arg {
                syn::FnArg::Captured(ref captured) => {
                    match captured.pat {
                        syn::Pat::Ident(ref pat_ident) => { pat_ident.ident.clone() }
                        _ => panic!("Unsupported argument type!")
                    }
                },
                _ => panic!("Unsupported argument type!")
            }
        };
        match pair {
            Pair::Punctuated(arg, p) => Pair::Punctuated(to_ident(arg), p),
            Pair::End(arg) => Pair::End(to_ident(arg))
        }
    });
    syn::punctuated::Punctuated::<syn::Ident, &Token!(,)>::from_iter(it)
}

fn return_type_is_self(output: &syn::ReturnType) -> bool {
    if let syn::ReturnType::Type(_, ty) = &output {
        if let syn::Type::Path(path) = ty.borrow() {
            let path = &path.path.segments;
            if path.len() == 1 {
                let name = path.first().unwrap().value().ident;
                if name == "Self" {
                    return true;
                }
            }
        }
    }
    false
}

fn wrap_from_inner(implem_tokens: quote::Tokens, output: &syn::ReturnType) -> quote::Tokens {
    if return_type_is_self(&output) {
        quote! { Self::from_inner(#implem_tokens) }
    } else {
        implem_tokens
    }
}

fn add_method(type_name: &syn::Ident, sig : &syn::MethodSig) -> quote::Tokens {
    let first_arg = arg_self_kind(sig.decl.inputs.iter().next());
    let arg_without_self = get_call_args(first_arg, &sig.decl.inputs);
    let method_name = sig.ident;
    let method_call = quote! { #method_name(#arg_without_self) };

    let mut implem_tokens = match first_arg {
        FirstArg::MutSelfValue => quote! { self.into_inner().#method_call },
        FirstArg::SelfValue => quote! { self.into_inner().#method_call },
        FirstArg::MutSelfRef => quote! { self.inner_mut().#method_call },
        FirstArg::SelfRef => quote! { self.inner().#method_call },
        FirstArg::NotSelf => quote! { #type_name.#method_call }
    };

    implem_tokens = wrap_from_inner(implem_tokens, &sig.decl.output);

    quote! {
        #sig {
            #implem_tokens
        }
    }
}

fn impl_delegable_trait(trait_item : &syn::ItemTrait) -> quote::Tokens {
    let name = trait_item.ident;
    let mut methods = quote::Tokens::new();

    for item in &trait_item.items {
        if let syn::TraitItem::Method(item_method) = item {
            methods.append_all(add_method(&name, &item_method.sig));
        }
        // skipping other kinds
    }

    quote_delegable(&name, &methods)
}

fn impl_delegable(ast: &syn::Item) -> quote::Tokens {
    let delegable_tokens =
    match ast {
        syn::Item::Trait(ref trait_item) => { impl_delegable_trait(trait_item) },
        syn::Item::Impl(ref impl_item) => { impl_delegable_impl(impl_item) },
        _ => {
            panic!("This macro can only be applied to a trait or an impl block.");
        }
    };
    quote! {
        #ast

        #delegable_tokens
    }
}

#[proc_macro_attribute]
pub fn delegable(_metadata: TokenStream, input : TokenStream) -> TokenStream {
    // Parse the string representation
    let ast : syn::Item = syn::parse(input).expect("failed to parse input.");

    // Build the impl
    let gen = impl_delegable(&ast);

    // Return the generated impl
    gen.into()
}
