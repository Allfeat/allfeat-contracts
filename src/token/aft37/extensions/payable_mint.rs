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
    aft37,
    traits::aft37::{extensions::metadata::*, extensions::payable_mint::*, *},
};
pub use aft37::{AFT37Impl, BalancesManager as _, Internal as _, InternalImpl as _};
use ink::{prelude::string::String, prelude::vec::Vec, storage::Mapping};
use openbrush::{
    contracts::ownable::*,
    traits::{AccountId, Balance, Storage},
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub price_per_mint: Mapping<Id, Balance>,
    pub max_supply: Mapping<Id, u32>,
}

pub trait AFT37PayableMintImpl:
    aft37::Internal
    + aft37::aft37::AFT37Impl
    + aft37::extensions::payable_mint::Internal
    + Storage<Data>
    + Storage<ownable::Data>
{
    /// Mints the given tokens
    fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), AFT37Error> {
        self.check_value(Self::env().transferred_value(), &ids_amounts)?;
        self.check_amount(&ids_amounts)?;

        self._mint_to(to, ids_amounts)?;
        Ok(())
    }

    /// Withdraws funds to contract owner
    #[openbrush::modifiers(only_owner)]
    fn withdraw(&mut self) -> Result<(), AFT37Error> {
        let balance = Self::env().balance();
        let current_balance = balance
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        let owner = self.data::<ownable::Data>().owner.get().unwrap().unwrap();
        Self::env()
            .transfer(owner, current_balance)
            .map_err(|_| AFT37Error::Custom(String::from("WithdrawalFailed")))?;
        Ok(())
    }

    /// Sets the max supply for the given token
    #[openbrush::modifiers(only_owner)]
    fn set_max_supply(&mut self, id: Id, max_supply: u32) -> Result<(), AFT37Error> {
        self.data::<Data>().max_supply.insert(id, &max_supply);

        Ok(())
    }

    /// Sets the price for the given token
    #[openbrush::modifiers(only_owner)]
    fn set_price(&mut self, id: Id, price: Balance) -> Result<(), AFT37Error> {
        self.data::<Data>().price_per_mint.insert(id, &price);

        Ok(())
    }

    /// Get token price
    fn price(&self, token_id: Id) -> Result<Balance, AFT37Error> {
        self.data::<Data>()
            .price_per_mint
            .get(token_id)
            .ok_or(AFT37Error::TokenNotExists)
    }

    /// Get max supply of tokens
    fn max_supply(&self, id: Id) -> Result<u32, AFT37Error> {
        self.data::<Data>()
            .max_supply
            .get(id)
            .ok_or(AFT37Error::TokenNotExists)
    }
}

pub trait Internal: Storage<Data> + aft37::Internal + aft37::aft37::AFT37Impl {
    /// Check if the transferred mint values is as expected
    fn check_value(
        &self,
        transferred_value: u128,
        ids_amounts: &[(Id, Balance)],
    ) -> Result<(), AFT37Error> {
        let value = ids_amounts
            .iter()
            .flat_map(|(id, amount)| {
                (amount).checked_mul(
                    self.data::<Data>()
                        .price_per_mint
                        .get(id)
                        .unwrap_or_default(),
                )
            })
            .sum::<Balance>();

        if transferred_value == value {
            Ok(())
        } else {
            Err(AFT37Error::Custom(String::from("BadMintValue")))
        }
    }

    /// Check amount of tokens to be minted
    fn check_amount(&self, ids_amounts: &[(Id, Balance)]) -> Result<(), AFT37Error> {
        for (id, mint_amount) in ids_amounts {
            if *mint_amount == 0 {
                return Err(AFT37Error::Custom(String::from("CannotMintZeroTokens")));
            }

            let token_supply = AFT37Impl::total_supply(self, Some(id.clone()));

            if let Some(amount) = token_supply.checked_add(*mint_amount) {
                if amount
                    > self
                        .data::<Data>()
                        .max_supply
                        .get(id)
                        .ok_or(AFT37Error::TokenNotExists)? as Balance
                {
                    return Err(AFT37Error::Custom(String::from("CollectionIsFull")));
                }
            }
        }
        Ok(())
    }
}
