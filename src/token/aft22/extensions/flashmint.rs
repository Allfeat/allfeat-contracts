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

pub use crate::{aft22, aft22::extensions::flashmint, traits::aft22::*};
pub use aft22::{AFT22Impl, Internal as _, InternalImpl as _};
pub use flashmint::Internal as _;
use ink::{
    env::CallFlags,
    prelude::{vec, vec::Vec},
};
pub use openbrush::contracts::traits::flashloan::*;
use openbrush::traits::{AccountId, Balance, Storage, String};

pub trait FlashLenderImpl: Storage<aft22::Data> + aft22::Internal + AFT22 + Internal {
    fn max_flashloan(&mut self, token: AccountId) -> Balance {
        if token == Self::env().account_id() {
            aft22::Internal::_max_supply(self) - self.total_supply()
        } else {
            0
        }
    }

    fn flash_fee(&self, token: AccountId, amount: Balance) -> Result<Balance, FlashLenderError> {
        if token != Self::env().account_id() {
            return Err(FlashLenderError::WrongTokenAddress);
        }
        Ok(self._get_fee(amount))
    }

    fn flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        let fee = self.flash_fee(token, amount)?;
        self._mint_to(receiver_account, amount)?;
        Internal::_on_flashloan(self, receiver_account, token, fee, amount, data)?;
        let this = Self::env().account_id();
        let current_allowance = self.allowance(receiver_account, this);
        if current_allowance < amount + fee {
            return Err(FlashLenderError::AllowanceDoesNotAllowRefund);
        }
        aft22::Internal::_approve_from_to(
            self,
            receiver_account,
            this,
            current_allowance - amount - fee,
        )?;

        let flash_fee_receiver = self._flash_fee_receiver();

        if let Some(fee_receiver) = flash_fee_receiver {
            if fee == 0 {
                aft22::Internal::_burn_from(self, receiver_account, amount + fee)?;
            } else {
                aft22::Internal::_burn_from(self, receiver_account, amount)?;
                aft22::Internal::_transfer_from_to(
                    self,
                    receiver_account,
                    fee_receiver,
                    fee,
                    vec![],
                )?;
            }
        }

        Ok(())
    }
}

pub trait Internal {
    fn _get_fee(&self, _amount: Balance) -> Balance;

    fn _on_flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError>;

    fn _flash_fee_receiver(&self) -> Option<AccountId>;
}

pub trait InternalImpl: Storage<aft22::Data> + Internal {
    fn _get_fee(&self, _amount: Balance) -> Balance {
        0
    }

    fn _on_flashloan(
        &mut self,
        receiver_account: AccountId,
        token: AccountId,
        fee: Balance,
        amount: Balance,
        data: Vec<u8>,
    ) -> Result<(), FlashLenderError> {
        let builder = FlashBorrowerRef::on_flashloan_builder(
            &receiver_account,
            Self::env().caller(),
            token,
            amount,
            fee,
            data,
        )
        .call_flags(CallFlags::default().set_allow_reentry(true));
        match builder.try_invoke() {
            Ok(Ok(Ok(_))) => Ok(()),
            Ok(Ok(Err(FlashBorrowerError::FlashloanRejected(message)))) => {
                Err(FlashLenderError::BorrowerRejected(message))
            }
            // Means unknown method
            Ok(Err(ink::LangError::CouldNotReadInput)) => Ok(()),
            // `NotCallable` means that the receiver is not a contract.
            Err(ink::env::Error::NotCallable) => Ok(()),
            _ => Err(FlashLenderError::BorrowerRejected(String::from(
                "Error while performing the `on_flashloan`",
            ))),
        }
    }

    fn _flash_fee_receiver(&self) -> Option<AccountId> {
        None
    }
}
