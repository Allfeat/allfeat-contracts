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
    aft37::extensions::batch,
    traits::aft37::{extensions::batch::*, *},
};
pub use aft37::{
    AFT37Impl, BalancesManager as _, BalancesManagerImpl as _, Internal as _, InternalImpl as _,
};
pub use batch::Internal as _;
use ink::prelude::vec::Vec;
use openbrush::traits::{AccountId, Balance, Storage};

pub trait AFT37BatchImpl: Internal + Storage<aft37::Data> {
    fn batch_transfer(
        &mut self,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), AFT37Error> {
        self._batch_transfer_from(Self::env().caller(), to, ids_amounts, data)
    }

    fn batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), AFT37Error> {
        self._batch_transfer_from(from, to, ids_amounts, data)
    }
}

pub trait Internal {
    fn _batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        data: Vec<u8>,
    ) -> Result<(), AFT37Error>;
}

pub trait InternalImpl:
    Internal + aft37::Internal + Storage<aft37::Data> + aft37::BalancesManager
{
    fn _batch_transfer_from(
        &mut self,
        from: AccountId,
        to: AccountId,
        ids_amounts: Vec<(Id, Balance)>,
        _: Vec<u8>,
    ) -> Result<(), AFT37Error> {
        let operator = Self::env().caller();

        for (id, value) in &ids_amounts {
            if from != operator && &self._get_allowance(&from, &operator, &Some(id)) < value {
                return Err(AFT37Error::NotAllowed);
            }
        }

        self._before_token_transfer(Some(&from), Some(&to), &ids_amounts)?;

        for (id, value) in &ids_amounts {
            self._decrease_allowance(&from, &operator, id, *value)?;

            self._decrease_balance(&from, id, value, false)?;
        }

        for (id, value) in &ids_amounts {
            self._increase_balance(&to, id, value, false)?;
        }

        self._after_token_transfer(Some(&from), Some(&to), &ids_amounts)?;

        self._emit_transfer_batch_event(Some(from), Some(to), ids_amounts);

        Ok(())
    }
}
