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

pub use crate::{
    aft22,
    aft22::extensions::wrapper,
    traits::aft22::{extensions::wrapper::*, *},
};
pub use aft22::{AFT22Impl, Internal as _, InternalImpl as _};
use ink::{env::CallFlags, prelude::vec::Vec};
use openbrush::traits::{AccountId, Balance, Storage, String};
pub use wrapper::Internal as _;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub underlying: Option<AccountId>,
}

pub trait AFT22WrapperImpl: Storage<Data> + Internal + aft22::Internal {
    fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
        self._deposit(amount)?;
        aft22::Internal::_mint_to(self, account, amount)
    }

    fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
        aft22::Internal::_burn_from(self, Self::env().caller(), amount)?;
        self._withdraw(account, amount)
    }
}

pub trait Internal {
    /// Mint wrapped token to cover any underlyingTokens that would have been transfered by mistake. Internal
    /// function that can be exposed with access control if desired.
    fn _recover(&mut self, account: AccountId) -> Result<Balance, AFT22Error>;

    /// helper function to transfer the underlying token from caller to the contract
    fn _deposit(&mut self, amount: Balance) -> Result<(), AFT22Error>;

    /// helper function to transfer the underlying token
    fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error>;

    /// helper function to get balance of underlying tokens in the contract
    fn _underlying_balance(&mut self) -> Balance;

    /// Initalize the wrapper token with defining the underlying AFT22 token
    ///
    /// `underlying` is the token to be wrapped
    fn _init(&mut self, underlying: AccountId);

    /// Getter for caller to `AFT22Wrapper` of `underlying`
    fn _underlying(&mut self) -> Option<AccountId>;
}

pub trait InternalImpl: Storage<Data> + Internal + aft22::Internal + AFT22 {
    fn _recover(&mut self, account: AccountId) -> Result<Balance, AFT22Error> {
        let value = Internal::_underlying_balance(self) - self.total_supply();
        aft22::Internal::_mint_to(self, account, value)?;
        Ok(value)
    }

    fn _deposit(&mut self, amount: Balance) -> Result<(), AFT22Error> {
        if let Some(underlying) = Internal::_underlying(self) {
            AFT22Ref::transfer_from_builder(
                &underlying,
                Self::env().caller(),
                Self::env().account_id(),
                amount,
                Vec::<u8>::new(),
            )
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .try_invoke()
            .unwrap()
            .unwrap()
        } else {
            Err(AFT22Error::Custom(String::from(
                "Underlying not initialized",
            )))
        }
    }

    fn _withdraw(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error> {
        if let Some(underlying) = Internal::_underlying(self) {
            AFT22Ref::transfer_builder(&underlying, account, amount, Vec::<u8>::new())
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .try_invoke()
                .unwrap()
                .unwrap()
        } else {
            Err(AFT22Error::Custom(String::from(
                "Underlying not initialized",
            )))
        }
    }

    fn _underlying_balance(&mut self) -> Balance {
        if let Some(underlying) = Internal::_underlying(self) {
            AFT22Ref::balance_of(&underlying, Self::env().account_id())
        } else {
            0
        }
    }

    fn _init(&mut self, underlying: AccountId) {
        self.data().underlying.set(&Some(underlying));
    }

    fn _underlying(&mut self) -> Option<AccountId> {
        self.data().underlying.get_or_default()
    }
}
