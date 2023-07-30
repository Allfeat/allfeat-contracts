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

use crate::aft34::ApprovalsKey;
pub use crate::{
    aft34,
    aft34::extensions::enumerable,
    traits::aft34::{extensions::enumerable::*, *},
};
pub use aft34::{
    AFT34Impl, BalancesManager as _, Internal as _, InternalImpl as _, Operator, Owner,
};
use openbrush::{
    storage::{Mapping, MultiMapping, TypeGuard},
    traits::{AccountId, Balance, Storage},
};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub token_owner: Mapping<Id, Owner>,
    pub operator_approvals: Mapping<(Owner, Operator, Option<Id>), (), ApprovalsKey>,
    pub balances: MultiMapping<Option<AccountId>, Id, EnumerableKey>,
}

pub struct EnumerableKey;

impl<'a> TypeGuard<'a> for EnumerableKey {
    type Type = &'a Option<&'a AccountId>;
}

pub trait BalancesManagerImpl: Storage<Data> {
    fn _balance_of(&self, owner: &Owner) -> u32 {
        self.data().balances.count(&Some(owner)) as u32
    }

    fn _increase_balance(&mut self, owner: &Owner, id: &Id, increase_supply: bool) {
        self.data().balances.insert(&Some(owner), id);
        if increase_supply {
            self.data().balances.insert(&None, id);
        }
    }

    fn _decrease_balance(&mut self, owner: &Owner, id: &Id, decrease_supply: bool) {
        self.data().balances.remove_value(&Some(owner), id);
        if decrease_supply {
            self.data().balances.remove_value(&None, id);
        }
    }

    fn _total_supply(&self) -> Balance {
        self.data().balances.count(&None)
    }

    fn _owner_of(&self, id: &Id) -> Option<AccountId> {
        self.data().token_owner.get(id)
    }

    fn _operator_approvals(
        &self,
        owner: &Owner,
        operator: &Operator,
        id: &Option<&Id>,
    ) -> Option<()> {
        self.data().operator_approvals.get(&(owner, operator, id))
    }

    fn _insert_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
        self.data()
            .operator_approvals
            .insert(&(owner, operator, id), &());
    }

    fn _remove_operator_approvals(&mut self, owner: &Owner, operator: &Operator, id: &Option<&Id>) {
        self.data()
            .operator_approvals
            .remove(&(owner, operator, id));
    }

    fn _insert_token_owner(&mut self, id: &Id, to: &AccountId) {
        self.data().token_owner.insert(id, to);
    }

    fn _remove_token_owner(&mut self, id: &Id) {
        self.data().token_owner.remove(id);
    }
}

pub trait AFT34EnumerableImpl: Storage<Data> {
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Result<Id, AFT34Error> {
        self.data()
            .balances
            .get_value(&Some(&owner), &index)
            .ok_or(AFT34Error::TokenNotExists)
    }

    fn token_by_index(&self, index: u128) -> Result<Id, AFT34Error> {
        self.data()
            .balances
            .get_value(&None, &index)
            .ok_or(AFT34Error::TokenNotExists)
    }
}
