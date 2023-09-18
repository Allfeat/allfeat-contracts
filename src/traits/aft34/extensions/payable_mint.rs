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

pub use crate::traits::aft34::Id;
/// Extension of [`PSP34`] that exposes the mint function
pub use crate::traits::errors::AFT34Error;
use ink::prelude::string::String as PreludeString;
use openbrush::traits::{AccountId, Balance};

#[openbrush::wrapper]
pub type AFT34PayableMintRef = dyn AFT34PayableMint;

#[openbrush::trait_definition]
pub trait AFT34PayableMint {
    #[ink(message, payable)]
    fn mint(&mut self, to: AccountId, mint_amount: u64) -> Result<(), AFT34Error>;

    #[ink(message)]
    fn withdraw(&mut self) -> Result<(), AFT34Error>;

    #[ink(message)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), AFT34Error>;

    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> Result<PreludeString, AFT34Error>;

    #[ink(message)]
    fn max_supply(&self) -> u64;

    #[ink(message)]
    fn price(&self) -> Balance;
}
