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

use std::collections::HashMap;
use syn::Block;
use syn::__private::quote::format_ident;
use synstructure::quote;

pub type IsDefault = bool;
pub type OverridenFnMap =
    HashMap<String, Vec<(String, (Box<Block>, Vec<syn::Attribute>, IsDefault))>>;

pub struct ImplArgs<'a> {
    pub map: &'a OverridenFnMap,
    pub items: &'a mut Vec<syn::Item>,
    pub imports: &'a mut HashMap<&'a str, syn::ItemUse>,
    pub overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
    pub storage_struct_name: String,
}

impl<'a> ImplArgs<'a> {
    pub fn new(
        map: &'a OverridenFnMap,
        items: &'a mut Vec<syn::Item>,
        imports: &'a mut HashMap<&'a str, syn::ItemUse>,
        overriden_traits: &'a mut HashMap<&'a str, syn::Item>,
        storage_struct_name: String,
    ) -> Self {
        Self {
            map,
            items,
            imports,
            overriden_traits,
            storage_struct_name,
        }
    }

    fn contract_name(&self) -> proc_macro2::Ident {
        format_ident!("{}", self.storage_struct_name)
    }

    fn vec_import(&mut self) {
        let vec_import = syn::parse2::<syn::ItemUse>(quote!(
            use ink::prelude::vec::Vec;
        ))
        .expect("Should parse");
        self.imports.insert("vec", vec_import);
    }
}

pub(crate) fn impl_aft34(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft34::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft34::Internal for #storage_struct_name {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id) {
                aft34::InternalImpl::_emit_transfer_event(self, from, to, id)
            }

            fn _emit_approval_event(&self, from: AccountId, to: AccountId, id: Option<Id>, approved: bool) {
                aft34::InternalImpl::_emit_approval_event(self, from, to, id, approved)
            }

            fn _approve_for(&mut self, to: AccountId, id: Option<Id>, approved: bool) -> Result<(), AFT34Error> {
                aft34::InternalImpl::_approve_for(self, to, id, approved)
            }

            fn _owner_of(&self, id: &Id) -> Option<AccountId> {
                aft34::InternalImpl::_owner_of(self, id)
            }

            fn _transfer_token(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), AFT34Error> {
                aft34::InternalImpl::_transfer_token(self, to, id, data)
            }

            fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), AFT34Error> {
                aft34::InternalImpl::_mint_to(self, to, id)
            }

            fn _burn_from(&mut self, from: AccountId, id: Id) -> Result<(), AFT34Error> {
                aft34::InternalImpl::_burn_from(self, from, id)
            }

            fn _allowance(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> bool {
                aft34::InternalImpl::_allowance(self, owner, operator, id)
            }

            fn _check_token_exists(&self, id: &Id) -> Result<AccountId, AFT34Error> {
                aft34::InternalImpl::_check_token_exists(self, id)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                id: &Id,
            ) -> Result<(), AFT34Error> {
                aft34::InternalImpl::_before_token_transfer(self, from, to, id)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                id: &Id,
            ) -> Result<(), AFT34Error> {
                aft34::InternalImpl::_after_token_transfer(self, from, to, id)
            }
        }
    ))
        .expect("Should parse");

    let aft34_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34Impl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut aft34 = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34 for #storage_struct_name {
            #[ink(message)]
            fn collection_id(&self) -> Id {
                AFT34Impl::collection_id(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> u32 {
                AFT34Impl::balance_of(self, owner)
            }

            #[ink(message)]
            fn owner_of(&self, id: Id) -> Option<AccountId> {
                AFT34Impl::owner_of(self, id)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> bool {
                AFT34Impl::allowance(self, owner, operator, id)
            }

            #[ink(message)]
            fn approve(&mut self, operator: AccountId, id: Option<Id>, approved: bool) -> Result<(), AFT34Error> {
                AFT34Impl::approve(self, operator, id, approved)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, id: Id, data: Vec<u8>) -> Result<(), AFT34Error> {
                AFT34Impl::transfer(self, to, id, data)
            }

            #[ink(message)]
            fn total_supply(&self) -> Balance {
                AFT34Impl::total_supply(self)
            }
        }
    ))
        .expect("Should parse");

    let aft34_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft34::BalancesManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut aft34_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft34::BalancesManager for #storage_struct_name {
            fn _balance_of(&self, owner: &Owner) -> u32 {
                aft34::BalancesManagerImpl::_balance_of(self, owner)
            }

            fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
                aft34::BalancesManagerImpl::_increase_balance(self, owner, id, increase_supply)
            }

            fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
                aft34::BalancesManagerImpl::_decrease_balance(self, owner, id, decrease_supply)
            }

            fn _total_supply(&self) -> u128 {
                aft34::BalancesManagerImpl::_total_supply(self)
            }

            fn _owner_of(&self, id: &Id) -> Option<AccountId> {
                aft34::BalancesManagerImpl::_owner_of(self, id)
            }

            fn _operator_approvals(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> Option<()> {
                aft34::BalancesManagerImpl::_operator_approvals(self, owner, operator, id)
            }

            fn _insert_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
                aft34::BalancesManagerImpl::_insert_operator_approvals(self, owner, operator, id)
            }

            fn _remove_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
                aft34::BalancesManagerImpl::_remove_operator_approvals(self, owner, operator, id)
            }

            fn _insert_token_owner(&mut self, id: &Id, to: &AccountId) {
                aft34::BalancesManagerImpl::_insert_token_owner(self, id, to)
            }

            fn _remove_token_owner(&mut self, id: &Id) {
                aft34::BalancesManagerImpl::_remove_token_owner(self, id)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft34::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT34", import);
    impl_args.vec_import();

    override_functions("aft34::BalancesManager", &mut aft34_balances, impl_args.map);
    override_functions("aft34::Internal", &mut internal, impl_args.map);
    override_functions("AFT34", &mut aft34, impl_args.map);

    // only insert this if it is not present
    impl_args
        .overriden_traits
        .entry("aft34::BalancesManager")
        .or_insert(syn::Item::Impl(aft34_balances));

    impl_args
        .overriden_traits
        .entry("aft34::BalancesManagerImpl")
        .or_insert(syn::Item::Impl(aft34_balances_impl));

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(aft34_impl));
    impl_args.items.push(syn::Item::Impl(aft34));
}

fn override_functions(trait_name: &str, implementation: &mut syn::ItemImpl, map: &OverridenFnMap) {
    if let Some(overrides) = map.get(trait_name) {
        // we will find which fns we wanna override
        for (fn_name, (fn_code, attributes, is_default)) in overrides {
            for item in implementation.items.iter_mut() {
                if let syn::ImplItem::Method(method) = item {
                    if &method.sig.ident.to_string() == fn_name {
                        if !is_default {
                            method.block = *fn_code.clone();
                        }
                        method.attrs.append(&mut attributes.to_vec());
                    }
                }
            }
        }
    }
}

pub(crate) fn impl_aft34_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34BurnableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, account: AccountId, id: Id) -> Result<(), AFT34Error> {
                AFT34BurnableImpl::burn(self, account, id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft34::extensions::burnable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT34Burnable", import);
    impl_args.vec_import();

    override_functions("AFT34Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_aft34_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34MintableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, account: AccountId, id: Id) -> Result<(), AFT34Error> {
                AFT34MintableImpl::mint(self, account, id)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft34::extensions::mintable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT34Mintable", import);
    impl_args.vec_import();

    override_functions("AFT34Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_aft34_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::Internal for #storage_struct_name {
            fn _emit_attribute_set_event(&self, id: Id, key: String, data: String) {
                metadata::InternalImpl::_emit_attribute_set_event(self, id, key, data)
            }

            fn _set_attribute(&mut self, id: Id, key: String, value: String) {
                metadata::InternalImpl::_set_attribute(self, id, key, value)
            }
        }
    ))
    .expect("Should parse");

    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34MetadataImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34Metadata for #storage_struct_name {
            #[ink(message)]
            fn get_attribute(&self, id: Id, key: String) -> Option<String> {
                AFT34MetadataImpl::get_attribute(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft34::extensions::metadata::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT34Metadata", import);
    impl_args.vec_import();

    override_functions("metadata::Internal", &mut internal, impl_args.map);
    override_functions("AFT34Mintable", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(metadata_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_aft34_enumerable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let enumerable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34EnumerableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut aft34_enumerable = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT34Enumerable for #storage_struct_name {
            #[ink(message)]
            fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, AFT34Error> {
                AFT34EnumerableImpl::owners_token_by_index(self, owner, index)
            }

            #[ink(message)]
            fn token_by_index(&self, index: u128) -> Result<Id, AFT34Error> {
                AFT34EnumerableImpl::token_by_index(self, index)
            }
        }

    ))
        .expect("Should parse");

    let aft34_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl enumerable::BalancesManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut aft34_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft34::BalancesManager for #storage_struct_name {
            fn _balance_of(&self, owner: &Owner) -> u32 {
                enumerable::BalancesManagerImpl::_balance_of(self, owner)
            }

            fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
                enumerable::BalancesManagerImpl::_increase_balance(self, owner, id, increase_supply)
            }

            fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
                enumerable::BalancesManagerImpl::_decrease_balance(self, owner, id, decrease_supply)
            }

            fn _total_supply(&self) -> u128 {
                enumerable::BalancesManagerImpl::_total_supply(self)
            }

            fn _owner_of(&self, id: &Id) -> Option<AccountId> {
                enumerable::BalancesManagerImpl::_owner_of(self, id)
            }

            fn _operator_approvals(&self, owner: &Owner, operator: &Operator, id: &Option<&Id>) -> Option<()> {
                enumerable::BalancesManagerImpl::_operator_approvals(self, owner, operator, id)
            }

            fn _insert_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
                enumerable::BalancesManagerImpl::_insert_operator_approvals(self, owner, operator, id)
            }

            fn _remove_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
                enumerable::BalancesManagerImpl::_remove_operator_approvals(self, owner, operator, id)
            }

            fn _insert_token_owner(&mut self, id: &Id, to: &AccountId) {
                enumerable::BalancesManagerImpl::_insert_token_owner(self, id, to)
            }

            fn _remove_token_owner(&mut self, id: &Id) {
                enumerable::BalancesManagerImpl::_remove_token_owner(self, id)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft34::extensions::enumerable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT34Enumerable", import);
    impl_args.vec_import();

    override_functions("aft34::BalancesManager", &mut aft34_balances, impl_args.map);
    override_functions("aft34Enumerable", &mut aft34_enumerable, impl_args.map);

    impl_args
        .overriden_traits
        .insert("aft34::BalancesManager", syn::Item::Impl(aft34_balances));
    impl_args.overriden_traits.insert(
        "aft34::BalancesManagerImpl",
        syn::Item::Impl(aft34_balances_impl),
    );

    impl_args.items.push(syn::Item::Impl(enumerable_impl));
    impl_args.items.push(syn::Item::Impl(aft34_enumerable));
}
