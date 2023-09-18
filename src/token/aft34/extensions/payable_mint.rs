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
    aft34,
    traits::aft34::{extensions::metadata::*, extensions::payable_mint::*, *},
};
pub use aft34::{
    AFT34Impl, BalancesManager as _, Internal as _, InternalImpl as _, Operator, Owner,
};
use ink::prelude::string::{String, ToString};
use openbrush::{
    contracts::ownable::*,
    traits::{AccountId, Balance, Storage},
};

use super::metadata::AFT34MetadataImpl;

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub last_token_id: u64,
    pub max_supply: u64,
    pub price_per_mint: Balance,
}

pub trait AFT34PayableMintImpl:
    aft34::Internal
    + aft34::aft34::AFT34Impl
    + aft34::extensions::metadata::Internal
    + aft34::extensions::payable_mint::Internal
    + aft34::extensions::metadata::AFT34MetadataImpl
    + Storage<Data>
    + Storage<ownable::Data>
{
    fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), AFT34Error> {
        self.check_value(Self::env().transferred_value(), mint_amount)?;
        self.check_amount(mint_amount)?;

        let next_to_mint = self.data::<Data>().last_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            self._mint_to(to, Id::U64(mint_id))?;
            self.data::<Data>().last_token_id += 1;
        }

        Ok(())
    }

    /// Withdraws funds to contract owner
    #[openbrush::modifiers(only_owner)]
    fn withdraw(&mut self) -> Result<(), AFT34Error> {
        let balance = Self::env().balance();
        let current_balance = balance
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        let owner = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
        Self::env()
            .transfer(owner, current_balance)
            .map_err(|_| AFT34Error::Custom(String::from("WithdrawalFailed")))?;
        Ok(())
    }

    /// Set new value for the baseUri
    #[openbrush::modifiers(only_owner)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), AFT34Error> {
        let id = AFT34Impl::collection_id(self);
        aft34::extensions::metadata::Internal::_set_attribute(
            self,
            id,
            String::from("baseUri"),
            uri,
        );

        Ok(())
    }

    /// Get URI from token ID
    fn token_uri(&self, token_id: u64) -> Result<String, AFT34Error> {
        let base_uri =
            AFT34MetadataImpl::get_attribute(self, Id::U64(token_id), String::from("baseUri"));
        let token_uri = base_uri
            .ok_or_else(|| AFT34Error::Custom(String::from("InvalidBaseUri")))?
            + &token_id.to_string()
            + &String::from(".json");
        Ok(token_uri)
    }

    /// Get max supply of tokens
    fn max_supply(&self) -> u64 {
        self.data::<Data>().max_supply
    }

    /// Get token price
    fn price(&self) -> Balance {
        self.data::<Data>().price_per_mint
    }
}

pub trait Internal: Storage<Data> + aft34::Internal {
    /// Check if the transferred mint values is as expected
    fn check_value(&self, transferred_value: u128, mint_amount: u64) -> Result<(), AFT34Error> {
        if let Some(value) = (mint_amount as u128).checked_mul(self.data::<Data>().price_per_mint) {
            if transferred_value == value {
                return Ok(());
            }
        }
        Err(AFT34Error::Custom(String::from("BadMintValue")))
    }

    /// Check amount of tokens to be minted
    fn check_amount(&self, mint_amount: u64) -> Result<(), AFT34Error> {
        if mint_amount == 0 {
            return Err(AFT34Error::Custom(String::from("CannotMintZeroTokens")));
        }
        if let Some(amount) = self.data::<Data>().last_token_id.checked_add(mint_amount) {
            if amount <= self.data::<Data>().max_supply {
                return Ok(());
            }
        }
        Err(AFT34Error::Custom(String::from("CollectionIsFull")))
    }

    /// Check if token is minted
    fn token_exists(&self, id: Id) -> Result<(), AFT34Error> {
        self._owner_of(&id).ok_or(AFT34Error::TokenNotExists)?;
        Ok(())
    }
}
