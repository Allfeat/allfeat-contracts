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

pub(crate) fn impl_aft22(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft22::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft22::Internal for #storage_struct_name {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, amount: Balance) {
                aft22::InternalImpl::_emit_transfer_event(self, from, to, amount)
            }

            fn _emit_approval_event(&self, owner: AccountId, spender: AccountId, amount: Balance) {
                aft22::InternalImpl::_emit_approval_event(self, owner, spender, amount)
            }

            fn _total_supply(&self) -> Balance {
                aft22::InternalImpl::_total_supply(self)
            }

            fn _balance_of(&self, owner: &AccountId) -> Balance {
                aft22::InternalImpl::_balance_of(self, owner)
            }

            fn _allowance(&self, owner: &AccountId, spender: &AccountId) -> Balance {
                aft22::InternalImpl::_allowance(self, owner, spender)
            }

            fn _transfer_from_to(
                &mut self,
                from: AccountId,
                to: AccountId,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), AFT22Error> {
                aft22::InternalImpl::_transfer_from_to(self, from, to, amount, data)
            }

            fn _approve_from_to(
                &mut self,
                owner: AccountId,
                spender: AccountId,
                amount: Balance,
            ) -> Result<(), AFT22Error> {
                aft22::InternalImpl::_approve_from_to(self, owner, spender, amount)
            }

            fn _mint_to(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
                aft22::InternalImpl::_mint_to(self, account, amount)
            }

            fn _burn_from(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
                aft22::InternalImpl::_burn_from(self, account, amount)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), AFT22Error> {
                aft22::InternalImpl::_before_token_transfer(self, from, to, amount)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                amount: &Balance,
            ) -> Result<(), AFT22Error> {
                aft22::InternalImpl::_after_token_transfer(self, from, to, amount)
            }
        }
    ))
        .expect("Should parse");

    let aft22_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22Impl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut aft22 = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22 for #storage_struct_name {
            #[ink(message)]
            fn total_supply(&self) -> Balance {
                AFT22Impl::total_supply(self)
            }

            #[ink(message)]
            fn balance_of(&self, owner: AccountId) -> Balance {
                AFT22Impl::balance_of(self, owner)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
                AFT22Impl::allowance(self, owner, spender)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, value: Balance, data: Vec<u8>) -> Result<(), AFT22Error> {
                AFT22Impl::transfer(self, to, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), AFT22Error> {
                AFT22Impl::transfer_from(self, from, to, value, data)
            }

            #[ink(message)]
            fn approve(&mut self, spender: AccountId, value: Balance) -> Result<(), AFT22Error> {
                AFT22Impl::approve(self, spender, value)
            }

            #[ink(message)]
            fn increase_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), AFT22Error> {
                AFT22Impl::increase_allowance(self, spender, delta_value)
            }

            #[ink(message)]
            fn decrease_allowance(&mut self, spender: AccountId, delta_value: Balance) -> Result<(), AFT22Error> {
                AFT22Impl::decrease_allowance(self, spender, delta_value)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft22::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT22", import);
    impl_args.vec_import();

    override_functions("aft22::Internal", &mut internal, impl_args.map);
    override_functions("AFT22", &mut aft22, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(aft22_impl));
    impl_args.items.push(syn::Item::Impl(aft22));
}

pub(crate) fn impl_aft22_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22MintableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
                AFT22MintableImpl::mint(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft22::extensions::mintable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT22Mintable", import);
    impl_args.vec_import();

    override_functions("AFT22Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_aft22_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22BurnableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
                AFT22BurnableImpl::burn(self, account, amount)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft22::extensions::burnable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT22Burnable", import);
    impl_args.vec_import();

    override_functions("AFT22Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_aft22_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22MetadataImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22Metadata for #storage_struct_name {
            #[ink(message)]
            fn token_name(&self) -> Option<String> {
                AFT22MetadataImpl::token_name(self)
            }

            #[ink(message)]
            fn token_symbol(&self) -> Option<String> {
                AFT22MetadataImpl::token_symbol(self)
            }

            #[ink(message)]
            fn token_decimals(&self) -> u8 {
                AFT22MetadataImpl::token_decimals(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft22::extensions::metadata::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT22Metadata", import);
    impl_args.vec_import();

    override_functions("AFT22Metadata", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(metadata_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_aft22_capped(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl capped::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl capped::Internal for #storage_struct_name {
            fn _init_cap(&mut self, cap: Balance) -> Result<(), AFT22Error> {
                capped::InternalImpl::_init_cap(self, cap)
            }

            fn _is_cap_exceeded(&self, amount: &Balance) -> bool {
                capped::InternalImpl::_is_cap_exceeded(self, amount)
            }

            fn _cap(&self) -> Balance {
                capped::InternalImpl::_cap(self)
            }
        }
    ))
    .expect("Should parse");

    let capped_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22CappedImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut capped = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22Capped for #storage_struct_name {
            #[ink(message)]
            fn cap(&self) -> Balance {
                AFT22CappedImpl::cap(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft22::extensions::capped::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT22Capped", import);
    impl_args.vec_import();

    override_functions("capped::Internal", &mut internal, impl_args.map);
    override_functions("AFT22Capped", &mut capped, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(capped_impl));
    impl_args.items.push(syn::Item::Impl(capped));
}

pub(crate) fn impl_aft22_wrapper(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl wrapper::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl wrapper::Internal for #storage_struct_name {
            fn _recover(&mut self, account: AccountId) -> Result<Balance, AFT22Error> {
                wrapper::InternalImpl::_recover(self, account)
            }

            fn _deposit(&mut self, amount: Balance) -> Result<(), AFT22Error> {
                wrapper::InternalImpl::_deposit(self, amount)
            }

            fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
                wrapper::InternalImpl::_withdraw(self, account, amount)
            }

            fn _underlying_balance(&mut self) -> Balance {
                wrapper::InternalImpl::_underlying_balance(self)
            }

            fn _init(&mut self, underlying: AccountId) {
                wrapper::InternalImpl::_init(self, underlying)
            }

            fn _underlying(&mut self) -> Option<AccountId> {
                wrapper::InternalImpl::_underlying(self)
            }
        }
    ))
    .expect("Should parse");

    let wrapper_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22WrapperImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut wrapper = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22Wrapper for #storage_struct_name {
            #[ink(message)]
            fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
                AFT22WrapperImpl::deposit_for(self, account, amount)
            }

            #[ink(message)]
            fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
                AFT22WrapperImpl::withdraw_to(self, account, amount)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft22::extensions::wrapper::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT22Wrapper", import);
    impl_args.vec_import();

    override_functions("wrapper::Internal", &mut internal, impl_args.map);
    override_functions("AFT22Wrapper", &mut wrapper, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(wrapper_impl));
    impl_args.items.push(syn::Item::Impl(wrapper));
}

pub(crate) fn impl_flashmint(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl flashmint::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl flashmint::Internal for #storage_struct_name {
            fn _get_fee(&self, amount: Balance) -> Balance {
                flashmint::InternalImpl::_get_fee(self, amount)
            }

            fn _on_flashloan(
                &mut self,
                receiver_account: AccountId,
                token: AccountId,
                fee: Balance,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), FlashLenderError> {
                flashmint::InternalImpl::_on_flashloan(self, receiver_account, token, fee, amount, data)
            }
        }
    ))
        .expect("Should parse");

    let flashlender_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl FlashLenderImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut flashlender = syn::parse2::<syn::ItemImpl>(quote!(
        impl FlashLender for #storage_struct_name {
            #[ink(message)]
            fn max_flashloan(&mut self, token: AccountId) -> Balance {
                FlashLenderImpl::max_flashloan(self, token)
            }

            #[ink(message)]
            fn flash_fee(&self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
                FlashLenderImpl::flash_fee(self, token, amount)
            }

            #[ink(message)]
            fn flashloan(
                &mut self,
                receiver_account: AccountId,
                token: AccountId,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), FlashLenderError> {
                FlashLenderImpl::flashloan(self, receiver_account, token, amount, data)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft22::extensions::flashmint::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("Flashmint", import);
    impl_args.vec_import();

    override_functions("flashmint::Internal", &mut internal, impl_args.map);
    override_functions("FlashLender", &mut flashlender, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(flashlender_impl));
    impl_args.items.push(syn::Item::Impl(flashlender));
}

pub(crate) fn impl_token_timelock(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl token_timelock::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl token_timelock::Internal for #storage_struct_name {
            fn _withdraw(&mut self, amount: Balance) -> Result<(), AFT22TokenTimelockError> {
                token_timelock::InternalImpl::_withdraw(self, amount)
            }

            fn _contract_balance(&mut self) -> Balance {
                token_timelock::InternalImpl::_contract_balance(self)
            }

            fn _init(
                &mut self,
                token: AccountId,
                beneficiary: AccountId,
                release_time: Timestamp,
            ) -> Result<(), AFT22TokenTimelockError> {
                token_timelock::InternalImpl::_init(self, token, beneficiary, release_time)
            }

            fn _token(&self) -> Option<AccountId> {
                token_timelock::InternalImpl::_token(self)
            }

            fn _beneficiary(&self) -> Option<AccountId> {
                token_timelock::InternalImpl::_beneficiary(self)
            }
        }
    ))
    .expect("Should parse");

    let timelock_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22TokenTimelockImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut timelock = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT22TokenTimelock for #storage_struct_name {
            #[ink(message)]
            fn token(&self) -> Option<AccountId> {
                AFT22TokenTimelockImpl::token(self)
            }

            #[ink(message)]
            fn beneficiary(&self) -> Option<AccountId> {
                AFT22TokenTimelockImpl::beneficiary(self)
            }

            #[ink(message)]
            fn release_time(&self) -> Timestamp {
                AFT22TokenTimelockImpl::release_time(self)
            }

            #[ink(message)]
            fn release(&mut self) -> Result<(), AFT22TokenTimelockError> {
                AFT22TokenTimelockImpl::release(self)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft22::utils::token_timelock::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT22TokenTimelock", import);

    override_functions("token_timelock::Internal", &mut internal, impl_args.map);
    override_functions("AFT22TokenTimelock", &mut timelock, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(timelock_impl));
    impl_args.items.push(syn::Item::Impl(timelock));
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

pub(crate) fn impl_aft37(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft37::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft37::Internal for #storage_struct_name {
            fn _emit_transfer_event(&self, from: Option<AccountId>, to: Option<AccountId>, id: Id, amount: Balance) {
                aft37::InternalImpl::_emit_transfer_event(self, from, to, id, amount)
            }

            fn _emit_transfer_batch_event(
                &self,
                from: Option<AccountId>,
                to: Option<AccountId>,
                ids_amounts: Vec<(Id, Balance)>,
            ) {
                aft37::InternalImpl::_emit_transfer_batch_event(self, from, to, ids_amounts)
            }

            fn _emit_approval_event(&self, owner: AccountId, operator: AccountId, id: Option<Id>, value: Balance) {
                aft37::InternalImpl::_emit_approval_event(self, owner, operator, id, value)
            }

            fn _mint_to(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), AFT37Error> {
                aft37::InternalImpl::_mint_to(self, to, ids_amounts)
            }

            fn _burn_from(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), AFT37Error> {
                aft37::InternalImpl::_burn_from(self, from, ids_amounts)
            }

            fn _transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                id: Id,
                amount: Balance,
                data: Vec<u8>,
            ) -> Result<(), AFT37Error> {
                aft37::InternalImpl::_transfer_from(self, from, to, id, amount, data)
            }

            fn _get_allowance(&self, account: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Balance {
                aft37::InternalImpl::_get_allowance(self, account, operator, id)
            }

            fn _approve_for(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), AFT37Error> {
                aft37::InternalImpl::_approve_for(self, operator, id, value)
            }

            fn _decrease_allowance(
                &mut self,
                owner: &AccountId,
                operator: &AccountId,
                id: &Id,
                value: Balance,
            ) -> Result<(), AFT37Error> {
                aft37::InternalImpl::_decrease_allowance(self, owner, operator, id, value)
            }

            fn _transfer_token(
                &mut self,
                from: &AccountId,
                to: &AccountId,
                id: Id,
                amount: Balance,
                data: &[u8],
            ) -> Result<(), AFT37Error> {
                aft37::InternalImpl::_transfer_token(self, from, to, id, amount, data)
            }

            fn _before_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                ids: &[(Id, Balance)],
            ) -> Result<(), AFT37Error> {
                aft37::InternalImpl::_before_token_transfer(self, from, to, ids)
            }

            fn _after_token_transfer(
                &mut self,
                from: Option<&AccountId>,
                to: Option<&AccountId>,
                ids: &[(Id, Balance)],
            ) -> Result<(), AFT37Error> {
                aft37::InternalImpl::_after_token_transfer(self, from, to, ids)
            }
        }

    ))
        .expect("Should parse");

    let aft37_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37Impl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut aft37 = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37 for #storage_struct_name {
            #[ink(message)]
            fn balance_of(&self, owner: AccountId, id: Option<Id>) -> Balance {
                AFT37Impl::balance_of(self, owner, id)
            }

            #[ink(message)]
            fn total_supply(&self, id: Option<Id>) -> Balance {
                AFT37Impl::total_supply(self, id)
            }

            #[ink(message)]
            fn allowance(&self, owner: AccountId, operator: AccountId, id: Option<Id>) -> Balance {
                AFT37Impl::allowance(self, owner, operator, id)
            }

            #[ink(message)]
            fn approve(&mut self, operator: AccountId, id: Option<Id>, value: Balance) -> Result<(), AFT37Error> {
                AFT37Impl::approve(self, operator, id, value)
            }

            #[ink(message)]
            fn transfer(&mut self, to: AccountId, id: Id, value: Balance, data: Vec<u8>) -> Result<(), AFT37Error> {
                AFT37Impl::transfer(self, to, id, value, data)
            }

            #[ink(message)]
            fn transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                id: Id,
                value: Balance,
                data: Vec<u8>,
            ) -> Result<(), AFT37Error> {
                AFT37Impl::transfer_from(self, from, to, id, value, data)
            }
        }
    ))
        .expect("Should parse");

    let aft37_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft37::BalancesManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut aft37_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft37::BalancesManager for #storage_struct_name {
            fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
                aft37::BalancesManagerImpl::_balance_of(self, owner, id)
            }

            fn _total_supply(&self, id: &Option<&Id>) -> Balance {
                aft37::BalancesManagerImpl::_total_supply(self, id)
            }

            fn _increase_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                mint: bool,
            ) -> Result<(), AFT37Error> {
                aft37::BalancesManagerImpl::_increase_balance(self, owner, id, amount, mint)
            }

            fn _decrease_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                burn: bool,
            ) -> Result<(), AFT37Error> {
                aft37::BalancesManagerImpl::_decrease_balance(self, owner, id, amount, burn)
            }

            fn _insert_operator_approvals(
                &mut self,
                owner: &AccountId,
                operator: &AccountId,
                id: &Option<&Id>,
                amount: &Balance,
            ) {
                aft37::BalancesManagerImpl::_insert_operator_approvals(self, owner, operator, id, amount)
            }

            fn _get_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Option<Balance> {
                aft37::BalancesManagerImpl::_get_operator_approvals(self, owner, operator, id)
            }
            fn _remove_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) {
                aft37::BalancesManagerImpl::_remove_operator_approvals(self, owner, operator, id)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft37::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT37", import);
    impl_args.vec_import();

    override_functions("aft37::BalancesManager", &mut aft37_balances, impl_args.map);
    override_functions("aft37::Internal", &mut internal, impl_args.map);
    override_functions("AFT37", &mut aft37, impl_args.map);

    // only insert this if it is not present
    impl_args
        .overriden_traits
        .entry("aft37::BalancesManager")
        .or_insert(syn::Item::Impl(aft37_balances));

    impl_args
        .overriden_traits
        .entry("aft37::BalancesManagerImpl")
        .or_insert(syn::Item::Impl(aft37_balances_impl));

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(aft37_impl));
    impl_args.items.push(syn::Item::Impl(aft37));
}

pub(crate) fn impl_aft37_batch(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl batch::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl batch::Internal for #storage_struct_name {
            fn _batch_transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                ids_amounts: Vec<(Id, Balance)>,
                data: Vec<u8>,
            ) -> Result<(), AFT37Error> {
                batch::InternalImpl::_batch_transfer_from(self, from, to, ids_amounts, data)
            }
        }
    ))
    .expect("Should parse");

    let batch_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37BatchImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut batch = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37Batch for #storage_struct_name {
            #[ink(message)]
            fn batch_transfer(
                &mut self,
                to: AccountId,
                ids_amounts: Vec<(Id, Balance)>,
                data: Vec<u8>,
            ) -> Result<(), AFT37Error> {
                AFT37BatchImpl::batch_transfer(self, to, ids_amounts, data)
            }

            #[ink(message)]
            fn batch_transfer_from(
                &mut self,
                from: AccountId,
                to: AccountId,
                ids_amounts: Vec<(Id, Balance)>,
                data: Vec<u8>,
            ) -> Result<(), AFT37Error> {
                AFT37BatchImpl::batch_transfer_from(self, from, to, ids_amounts, data)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft37::extensions::batch::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT37Batch", import);
    impl_args.vec_import();

    override_functions("batch::Internal", &mut internal, impl_args.map);
    override_functions("AFT37Batch", &mut batch, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(batch_impl));
    impl_args.items.push(syn::Item::Impl(batch));
}

pub(crate) fn impl_aft37_burnable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let burnable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37BurnableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut burnable = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37Burnable for #storage_struct_name {
            #[ink(message)]
            fn burn(&mut self, from: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), AFT37Error> {
                AFT37BurnableImpl::burn(self, from, ids_amounts)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft37::extensions::burnable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT37Burnable", import);
    impl_args.vec_import();

    override_functions("AFT37Burnable", &mut burnable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(burnable_impl));
    impl_args.items.push(syn::Item::Impl(burnable));
}

pub(crate) fn impl_aft37_metadata(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let internal_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::InternalImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut internal = syn::parse2::<syn::ItemImpl>(quote!(
        impl metadata::Internal for #storage_struct_name {
            fn _emit_attribute_set_event(&self, id: &Id, key: &String, data: &String) {
                metadata::InternalImpl::_emit_attribute_set_event(self, id, key, data);
            }

            fn _set_attribute(&mut self, id: &Id, key: &String, data: &String) -> Result<(), AFT37Error> {
                metadata::InternalImpl::_set_attribute(self, id, key, data)
            }

            fn _get_attribute(&self, id: &Id, key: &String) -> Option<String> {
                metadata::InternalImpl::_get_attribute(self, id, key)
            }
        }
    ))
        .expect("Should parse");

    let metadata_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37MetadataImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut metadata = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37Metadata for #storage_struct_name {
            #[ink(message)]
            fn get_attribute(&self, id: Id, key: String) -> Option<String> {
                AFT37MetadataImpl::get_attribute(self, id, key)
            }
        }
    ))
    .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft37::extensions::metadata::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT37Metadata", import);
    impl_args.vec_import();

    override_functions("metadata::Internal", &mut internal, impl_args.map);
    override_functions("AFT37Metadata", &mut metadata, impl_args.map);

    impl_args.items.push(syn::Item::Impl(internal_impl));
    impl_args.items.push(syn::Item::Impl(internal));
    impl_args.items.push(syn::Item::Impl(metadata_impl));
    impl_args.items.push(syn::Item::Impl(metadata));
}

pub(crate) fn impl_aft37_mintable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let mintable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37MintableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut mintable = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37Mintable for #storage_struct_name {
            #[ink(message)]
            fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), AFT37Error> {
                AFT37MintableImpl::mint(self, to, ids_amounts)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft37::extensions::mintable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT37Mintable", import);
    impl_args.vec_import();

    override_functions("AFT37Mintable", &mut mintable, impl_args.map);

    impl_args.items.push(syn::Item::Impl(mintable_impl));
    impl_args.items.push(syn::Item::Impl(mintable));
}

pub(crate) fn impl_aft37_enumerable(impl_args: &mut ImplArgs) {
    let storage_struct_name = impl_args.contract_name();
    let enumerable_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37EnumerableImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut aft37_enumerable = syn::parse2::<syn::ItemImpl>(quote!(
        impl AFT37Enumerable for #storage_struct_name {
            #[ink(message)]
            fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Option<Id> {
                AFT37EnumerableImpl::owners_token_by_index(self, owner, index)
            }

            #[ink(message)]
            fn token_by_index(&self, index: u128) -> Option<Id> {
                AFT37EnumerableImpl::token_by_index(self, index)
            }
        }
    ))
    .expect("Should parse");

    let aft37_balances_impl = syn::parse2::<syn::ItemImpl>(quote!(
        impl enumerable::BalancesManagerImpl for #storage_struct_name {}
    ))
    .expect("Should parse");

    let mut aft37_balances = syn::parse2::<syn::ItemImpl>(quote!(
        impl aft37::BalancesManager for #storage_struct_name {
            fn _balance_of(&self, owner: &AccountId, id: &Option<&Id>) -> Balance {
                enumerable::BalancesManagerImpl::_balance_of(self, owner, id)
            }

            fn _total_supply(&self, id: &Option<&Id>) -> Balance {
                enumerable::BalancesManagerImpl::_total_supply(self, id)
            }

            fn _increase_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                mint: bool,
            ) -> Result<(), AFT37Error> {
                enumerable::BalancesManagerImpl::_increase_balance(self, owner, id, amount, mint)
            }

            fn _decrease_balance(
                &mut self,
                owner: &AccountId,
                id: &Id,
                amount: &Balance,
                burn: bool,
            ) -> Result<(), AFT37Error> {
                enumerable::BalancesManagerImpl::_decrease_balance(self, owner, id, amount, burn)
            }

            fn _insert_operator_approvals(
                &mut self,
                owner: &AccountId,
                operator: &AccountId,
                id: &Option<&Id>,
                amount: &Balance,
            ) {
                enumerable::BalancesManagerImpl::_insert_operator_approvals(self, owner, operator, id, amount)
            }

            fn _get_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>) -> Option<Balance> {
                enumerable::BalancesManagerImpl::_get_operator_approvals(self, owner, operator, id)
            }

            fn _remove_operator_approvals(&self, owner: &AccountId, operator: &AccountId, id: &Option<&Id>){
                enumerable::BalancesManagerImpl::_remove_operator_approvals(self, owner, operator, id)
            }
        }
    ))
        .expect("Should parse");

    let import = syn::parse2::<syn::ItemUse>(quote!(
        use allfeat_contracts::aft37::extensions::enumerable::*;
    ))
    .expect("Should parse");
    impl_args.imports.insert("AFT37Enumerable", import);
    impl_args.vec_import();

    override_functions("aft37::BalancesManager", &mut aft37_balances, impl_args.map);
    override_functions("AFT37Enumerable", &mut aft37_enumerable, impl_args.map);

    impl_args
        .overriden_traits
        .insert("aft37::BalancesManager", syn::Item::Impl(aft37_balances));
    impl_args.overriden_traits.insert(
        "aft37::BalancesManagerImpl",
        syn::Item::Impl(aft37_balances_impl),
    );

    impl_args.items.push(syn::Item::Impl(enumerable_impl));
    impl_args.items.push(syn::Item::Impl(aft37_enumerable));
}
