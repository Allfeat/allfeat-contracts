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

/// Extension of [`AFT22`] which allows the beneficiary to extract tokens after given time
pub use crate::{
    aft22,
    aft22::utils::token_timelock,
    traits::aft22::{utils::token_timelock::*, *},
};
pub use aft22::{AFT22Impl, Internal as _, InternalImpl as _};
use ink::{env::CallFlags, prelude::vec::Vec};
use openbrush::traits::{AccountId, Balance, Storage, Timestamp};
pub use token_timelock::{AFT22TokenTimelockImpl as _, Internal as _, InternalImpl as _};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    token: Option<AccountId>,
    #[lazy]
    beneficiary: Option<AccountId>,
    #[lazy]
    release_time: Timestamp,
}

pub trait AFT22TokenTimelockImpl: Storage<Data> + Internal {
    /// Returns the token address
    fn token(&self) -> Option<AccountId> {
        self._token()
    }

    /// Returns the beneficiary of the tokens
    fn beneficiary(&self) -> Option<AccountId> {
        self._beneficiary()
    }

    /// Returns the timestamp when the tokens are released
    fn release_time(&self) -> Timestamp {
        self.data().release_time.get_or_default()
    }

    /// Transfers the tokens held by timelock to the beneficairy
    fn release(&mut self) -> Result<(), AFT22TokenTimelockError> {
        if Self::env().block_timestamp() < self.data().release_time.get_or_default() {
            return Err(AFT22TokenTimelockError::CurrentTimeIsBeforeReleaseTime);
        }
        let amount = self._contract_balance();
        if amount == 0 {
            return Err(AFT22TokenTimelockError::NoTokensToRelease);
        }
        self._withdraw(amount)
    }
}

pub trait Internal {
    /// Helper function to withdraw tokens
    fn _withdraw(&mut self, amount: Balance) -> Result<(), AFT22TokenTimelockError>;

    /// Helper function to return balance of the contract
    fn _contract_balance(&mut self) -> Balance;

    /// Initializes the contract
    fn _init(
        &mut self,
        token: AccountId,
        beneficiary: AccountId,
        release_time: Timestamp,
    ) -> Result<(), AFT22TokenTimelockError>;

    fn _token(&self) -> Option<AccountId>;

    fn _beneficiary(&self) -> Option<AccountId>;
}

pub trait InternalImpl: Storage<Data> + Internal {
    fn _withdraw(&mut self, amount: Balance) -> Result<(), AFT22TokenTimelockError> {
        if let Some(beneficiary) = Internal::_beneficiary(self) {
            if let Some(token) = Internal::_token(self) {
                AFT22Ref::transfer_builder(&token, beneficiary, amount, Vec::<u8>::new())
                    .call_flags(CallFlags::default().set_allow_reentry(true))
                    .try_invoke()
                    .unwrap()
                    .unwrap()?;
                Ok(())
            } else {
                Err(AFT22TokenTimelockError::TokenZeroAddress)
            }
        } else {
            Err(AFT22TokenTimelockError::BeneficiaryZeroAddress)
        }
    }

    fn _contract_balance(&mut self) -> Balance {
        if let Some(token) = Internal::_token(self) {
            AFT22Ref::balance_of(&token, Self::env().account_id())
        } else {
            0
        }
    }

    fn _init(
        &mut self,
        token: AccountId,
        beneficiary: AccountId,
        release_time: Timestamp,
    ) -> Result<(), AFT22TokenTimelockError> {
        if release_time <= Self::env().block_timestamp() {
            return Err(AFT22TokenTimelockError::ReleaseTimeIsBeforeCurrentTime);
        }
        self.data().token.set(&Some(token));
        self.data().beneficiary.set(&Some(beneficiary));
        self.data().release_time.set(&release_time);
        Ok(())
    }

    fn _token(&self) -> Option<AccountId> {
        self.data().token.get_or_default()
    }

    fn _beneficiary(&self) -> Option<AccountId> {
        self.data().beneficiary.get_or_default()
    }
}
