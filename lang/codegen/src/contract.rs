use crate::{internal::*, metadata::LockedTrait};
use openbrush_lang_codegen::trait_definition;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Item;

pub fn generate(_attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    if skip() {
        return quote! {};
    }
    let input: TokenStream = ink_module;
    let attrs: TokenStream = _attrs;
    let mut module = syn::parse2::<syn::ItemMod>(input).expect("Can't parse contract module");
    let (braces, mut items) = match module.content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}",
                "out-of-line allfeat modules are not supported, use `#[allfeat::contract] mod name {{ ... }}`",
            )
        }
    };

    // First, we need to consume all traits and update metadata file.
    // After, we can consume all other stuff.
    items = consume_traits(items);

    let generated_items = generate_impls(items);

    module.content = Some((braces, generated_items));

    quote! {
        #[::ink::contract(#attrs)]
        #module
    }
}

fn consume_traits(items: Vec<Item>) -> Vec<Item> {
    let mut result: Vec<Item> = vec![];
    items.into_iter().for_each(|mut item| {
        if let Item::Trait(item_trait) = &mut item {
            if is_attr(&item_trait.attrs, "trait_definition") {
                item_trait.attrs = remove_attr(&item_trait.attrs, "trait_definition");

                let stream: TokenStream =
                    trait_definition::generate(TokenStream::new(), item_trait.to_token_stream());
                let mod_item = syn::parse2::<syn::ItemMod>(quote! {
                    mod jora {
                        #stream
                    }
                })
                .expect("Can't parse generated trait definitions");

                let (_, mut generated_items) = mod_item.content.unwrap();
                result.append(&mut generated_items);
            } else {
                result.push(item);
            }
        } else {
            result.push(item);
        }
    });

    result
}

fn generate_impls(mut items: Vec<Item>) -> Vec<Item> {
    let mut generated_items: Vec<Item> = vec![];
    items.iter_mut().for_each(|mut item| {
        if let Item::Impl(item_impl) = &mut item {
            if let Some((_, trait_path, _)) = item_impl.trait_.clone() {
                let trait_ident = trait_path
                    .segments
                    .last()
                    .expect("Trait path is empty")
                    .ident
                    .clone();
                let trait_lock = LockedTrait::new(trait_ident.to_string());
                if let Some(trait_definition) = &trait_lock.trait_definition {
                    let mut generated_impls =
                        impl_external_trait(item_impl.clone(), &trait_path, trait_definition);
                    generated_items.append(&mut generated_impls);
                    return;
                }
            }

            generated_items.push(syn::Item::from(item_impl.clone()));
        } else {
            generated_items.push(item.clone());
        }
    });

    generated_items
}
