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

pub use crate::traits::aft37::Id;
/// Extension of [`PSP37`] that exposes the mint function
pub use crate::traits::errors::AFT37Error;
use ink::prelude::vec::Vec;
use openbrush::traits::{AccountId, Balance};

#[openbrush::wrapper]
pub type AFT37PayableMintRef = dyn AFT37PayableMint;

#[openbrush::trait_definition]
pub trait AFT37PayableMint {
    #[ink(message, payable)]
    fn mint(&mut self, to: AccountId, ids_amounts: Vec<(Id, Balance)>) -> Result<(), AFT37Error>;

    #[ink(message)]
    fn withdraw(&mut self) -> Result<(), AFT37Error>;

    #[ink(message)]
    fn max_supply(&self, id: Id) -> Result<u32, AFT37Error>;

    #[ink(message)]
    fn set_max_supply(&mut self, id: Id, max_supply: u32) -> Result<(), AFT37Error>;

    #[ink(message)]
    fn price(&self, token_id: Id) -> Result<Balance, AFT37Error>;

    #[ink(message)]
    fn set_price(&mut self, id: Id, price: Balance) -> Result<(), AFT37Error>;
}
