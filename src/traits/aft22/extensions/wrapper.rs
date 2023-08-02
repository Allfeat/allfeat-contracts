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
/// Extension of [`AFT22`] which supports token wrapping
pub use crate::traits::aft22::*;
pub use crate::traits::errors::AFT22Error;

use openbrush::traits::{AccountId, Balance};

/// The idea of AFT22Wrapper is that it is AFT22 by itself.
/// Wrapper only adds 2 additional methods for depositing and withdrawing.
#[openbrush::wrapper]
pub type AFT22WrapperRef = dyn AFT22Wrapper + AFT22;

#[openbrush::trait_definition]
pub trait AFT22Wrapper: AFT22 {
    /// Allow a user to deposit `amount` of underlying tokens and mint `amount` of the wrapped tokens to `account`
    #[ink(message)]
    fn deposit_for(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error>;

    /// Allow a user to burn `amount` of wrapped tokens and withdraw the corresponding number of underlying tokens to `account`
    #[ink(message)]
    fn withdraw_to(&mut self, account: AccountId, amount: Balance) -> Result<(), AFT22Error>;
}
