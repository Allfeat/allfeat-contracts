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

/// Extension of [`AFT37`] that adds enumerability of all the token ids in the contract as well
/// as all token ids owned by each account.
pub use crate::traits::aft37::*;
use openbrush::traits::AccountId;

#[openbrush::wrapper]
pub type AFT37EnumerableRef = dyn AFT37Enumerable + AFT37;

#[openbrush::trait_definition]
pub trait AFT37Enumerable: AFT37 {
    /// Returns a token `Id` owned by `owner` at a given `index` of its token list.
    /// Use along with `balance_of` to enumerate all of ``owner``'s tokens.
    ///
    /// The start index is zero.
    #[ink(message)]
    fn owners_token_by_index(&self, owner: AccountId, index: u128) -> Option<Id>;

    /// Returns a token `Id` at a given `index` of all the tokens stored by the contract.
    /// Use along with `total_supply` to enumerate all tokens.
    ///
    /// The start index is zero.
    #[ink(message)]
    fn token_by_index(&self, index: u128) -> Option<Id>;
}
