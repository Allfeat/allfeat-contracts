// Copyright (c) 2022-2023 Allfeat labs
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::implementations::*;
use crate::{internal, internal::*};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{Item, Path};

pub fn generate(attrs: TokenStream, ink_module: TokenStream) -> TokenStream {
    if internal::skip() {
        return quote! {};
    }
    let input: TokenStream = ink_module;

    // map attribute args to default contract names
    let args = syn::parse2::<AttributeArgs>(attrs)
        .expect("No default contracts to implement provided")
        .iter()
        .map(|arg| match arg {
            NestedMeta::Path(method) => method.to_token_stream().to_string().replace(' ', ""),
            _ => panic!("Expected names of Allfeat traits to implement in the contract!"),
        })
        .collect::<Vec<String>>();

    let mut module = syn::parse2::<syn::ItemMod>(input).expect("Can't parse contract module");
    let (braces, items) = match module.clone().content {
        Some((brace, items)) => (brace, items),
        None => {
            panic!(
                "{}",
                "out-of-line allfeat modules are not supported, use `#[implementation] mod name {{ ... }}`",
            )
        }
    };

    // name of struct for which we will implement the traits
    let ident = extract_storage_struct_name(&items);
    // we will look for overriden functions and remove them from the mod
    let (map, mut items) = consume_overriders(items);

    // to save importing of stuff by users
    let mut imports = HashMap::<&str, syn::ItemUse>::default();
    // if multiple contracts are using the same trait implemented differently we override it this way
    let mut overriden_traits = HashMap::<&str, syn::Item>::default();

    let mut impl_args = ImplArgs::new(&map, &mut items, &mut imports, &mut overriden_traits, ident);
    let is_capped = args.contains(&"AFT22Capped".to_string());

    for to_implement in args.clone() {
        match to_implement.as_str() {
            "AFT22" => impl_aft22(&mut impl_args, is_capped),
            "AFT22Mintable" => impl_aft22_mintable(&mut impl_args),
            "AFT22Burnable" => impl_aft22_burnable(&mut impl_args),
            "AFT22Metadata" => impl_aft22_metadata(&mut impl_args),
            "AFT22Capped" => impl_aft22_capped(&mut impl_args),
            "AFT22Wrapper" => impl_aft22_wrapper(&mut impl_args),
            "Flashmint" => impl_flashmint(&mut impl_args),
            "AFT22TokenTimelock" => impl_token_timelock(&mut impl_args),
            "AFT34" => impl_aft34(&mut impl_args),
            "AFT34Burnable" => impl_aft34_burnable(&mut impl_args),
            "AFT34Metadata" => impl_aft34_metadata(&mut impl_args),
            "AFT34Enumerable" => impl_aft34_enumerable(&mut impl_args),
            "AFT34Mintable" => impl_aft34_mintable(&mut impl_args),
            "AFT34PayableMint" => impl_aft34_payable_mint(&mut impl_args),
            "AFT34URIStorage" => impl_aft34_uri_storage(&mut impl_args),
            "AFT37" => impl_aft37(&mut impl_args),
            "AFT37Batch" => impl_aft37_batch(&mut impl_args),
            "AFT37Burnable" => impl_aft37_burnable(&mut impl_args),
            "AFT37Metadata" => impl_aft37_metadata(&mut impl_args),
            "AFT37Mintable" => impl_aft37_mintable(&mut impl_args),
            "AFT37Enumerable" => impl_aft37_enumerable(&mut impl_args),
            _ => panic!("allfeat_contracts::implementation({to_implement}) not implemented!"),
        }
    }

    if args.contains(&String::from("AFT22")) {
        impl_aft22_transfer(&mut impl_args, is_capped);
    }

    cleanup_imports(impl_args.imports);

    // add the imports
    impl_args.items.append(
        &mut impl_args
            .imports
            .values()
            .cloned()
            .map(syn::Item::Use)
            .collect(),
    );

    // add overriden traits
    impl_args
        .items
        .append(&mut impl_args.overriden_traits.values().cloned().collect());

    module.content = Some((braces, items));

    quote! {
        #module
    }
}

fn cleanup_imports(imports: &mut HashMap<&str, syn::ItemUse>) {
    // we will remove unnecessary imports
    let aft22_impls = vec![
        "AFT22Mintable",
        "AFT22Burnable",
        "AFT22Capped",
        "AFT22Metadata",
        "AFT22Wrapper",
        "Flashmint",
    ];
    check_and_remove_import("AFT22", aft22_impls, imports);

    let aft34_impls = vec![
        "AFT34Mintable",
        "AFT34PayableMint",
        "AFT34Burnable",
        "AFT34Metadata",
        "AFT34Enumerable",
        "AFT34URIStorage",
    ];
    check_and_remove_import("AFT34", aft34_impls, imports);

    let aft37_impls = vec![
        "AFT37Batch",
        "AFT37Burnable",
        "AFT37Metadata",
        "AFT37Mintable",
        "AFT37Enumerable",
    ];
    check_and_remove_import("AFT37", aft37_impls, imports);

    let access_impls = vec!["AccessControlEnumerable", "TimelockController"];
    check_and_remove_import("AccessControl", access_impls, imports);

    check_and_remove_import("Diamond", vec!["DiamondLoupe"], imports);
}

fn check_and_remove_import(
    name_to_check: &str,
    to_check: Vec<&str>,
    imports: &mut HashMap<&str, syn::ItemUse>,
) {
    if to_check.iter().any(|name| imports.contains_key(name)) {
        imports.remove(name_to_check);
    }
}

// this method consumes override annotated methods and returns them mapped to code and the mod without them
// we will later override the methods
fn consume_overriders(items: Vec<syn::Item>) -> (OverridenFnMap, Vec<syn::Item>) {
    let mut map = HashMap::new();
    let mut result: Vec<syn::Item> = vec![];
    items.into_iter().for_each(|mut item| {
        if let Item::Fn(item_fn) = &mut item {
            if is_attr(&item_fn.attrs, "overrider") || is_attr(&item_fn.attrs, "default_impl") {
                let attr_name = if is_attr(&item_fn.attrs, "overrider") {
                    "overrider"
                } else {
                    "default_impl"
                };
                let fn_name = item_fn.sig.ident.to_string();
                let code = item_fn.block.clone();
                let mut attributes = item_fn.attrs.clone();

                // we will remove the overrider attribute since some other attributes might be interesting to us
                let to_remove_idx = attributes
                    .iter()
                    .position(|attr| is_attr(&[attr.clone()], attr_name))
                    .expect("No {attr_name} attribute found!");
                let overrider_attribute = attributes.remove(to_remove_idx);

                let trait_name = overrider_attribute
                    .parse_args::<Path>()
                    .expect("Expected overriden trait identifier")
                    .to_token_stream()
                    .to_string()
                    .replace(' ', "");

                let mut vec = map.get(&trait_name).unwrap_or(&vec![]).clone();
                vec.push((fn_name, (code, attributes, attr_name == "default_impl")));
                map.insert(trait_name, vec.to_vec());
            } else {
                result.push(item);
            }
        } else {
            result.push(item);
        }
    });

    (map, result)
}

fn extract_storage_struct_name(items: &[syn::Item]) -> String {
    let contract_storage_struct = items
        .iter()
        .find(|item| {
            if let Item::Struct(structure) = item {
                let ink_attr_maybe = structure
                    .attrs
                    .iter()
                    .cloned()
                    .find(|attr| is_attr(&[attr.clone()], "ink"));

                if let Some(ink_attr) = ink_attr_maybe {
                    if let Ok(path) = ink_attr.parse_args::<Path>() {
                        return path.to_token_stream().to_string() == "storage";
                    }
                }
                false
            } else {
                false
            }
        })
        .expect("Contract storage struct not found!");
    match contract_storage_struct {
        Item::Struct(structure) => structure.ident.to_string(),
        _ => unreachable!("Only Item::Struct allowed here"),
    }
}
