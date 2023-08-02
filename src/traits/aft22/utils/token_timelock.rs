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

pub use crate::traits::errors::AFT22TokenTimelockError;
use openbrush::traits::{AccountId, Timestamp};

#[openbrush::wrapper]
pub type AFT22TokenTimelockRef = dyn AFT22TokenTimelock;

#[openbrush::trait_definition]
pub trait AFT22TokenTimelock {
    /// Returns the token address
    #[ink(message)]
    fn token(&self) -> Option<AccountId>;

    /// Returns the beneficiary of the tokens
    #[ink(message)]
    fn beneficiary(&self) -> Option<AccountId>;

    /// Returns the timestamp when the tokens are released
    #[ink(message)]
    fn release_time(&self) -> Timestamp;

    /// Transfers the tokens held by timelock to the beneficairy
    #[ink(message)]
    fn release(&mut self) -> Result<(), AFT22TokenTimelockError>;
}
