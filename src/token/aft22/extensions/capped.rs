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
    aft22::extensions::capped,
    traits::aft22::{extensions::capped::*, *},
};
pub use aft22::{AFT22Impl, Internal as _, InternalImpl as _};
pub use capped::Internal as _;
use openbrush::traits::{AccountId, Balance, Storage, String};

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    #[lazy]
    pub cap: Balance,
}

pub trait AFT22CappedImpl: Internal {
    fn cap(&self) -> Balance {
        self._cap()
    }
}

pub trait Internal {
    /// Initializes the token's cap
    fn _init_cap(&mut self, cap: Balance) -> Result<(), AFT22Error>;

    fn _is_cap_exceeded(&self, amount: &Balance) -> bool;

    fn _cap(&self) -> Balance;
}

pub trait InternalImpl: Storage<Data> + Internal + AFT22 {
    fn _init_cap(&mut self, cap: Balance) -> Result<(), AFT22Error> {
        if cap == 0 {
            return Err(AFT22Error::Custom(String::from("Cap must be above 0")));
        }
        self.data().cap.set(&cap);
        Ok(())
    }

    fn _is_cap_exceeded(&self, amount: &Balance) -> bool {
        if self.total_supply() + amount > Internal::_cap(self) {
            return true;
        }
        false
    }

    fn _cap(&self) -> Balance {
        self.data().cap.get_or_default()
    }
}

pub trait AFT22TransferImpl: Internal {
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), AFT22Error> {
        if _from.is_none() && _to.is_some() && Internal::_is_cap_exceeded(self, _amount) {
            return Err(AFT22Error::Custom(String::from("Cap exceeded")));
        }

        Ok(())
    }

    fn _after_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        _to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), AFT22Error> {
        Ok(())
    }
}
