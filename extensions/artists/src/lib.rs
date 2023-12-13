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

#![cfg_attr(not(feature = "std"), no_std, no_main)]

use genres_registry::MusicGenre;
use ink::env::{DefaultEnvironment, Environment};
use ink::prelude::vec::Vec;
use scale::{Decode, Encode};

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;
pub type Hash = <DefaultEnvironment as Environment>::Hash;

/// Implementation of all functions contained in the pallet_artists available to ink! contracts.
pub struct ArtistExtension;

impl ArtistExtension {
    // Chain State Queries
    pub fn artist(account_id: AccountId) -> Option<ArtistDataOutput> {
        ::ink::env::chain_extension::ChainExtensionMethod::build(0051u32)
            .input::<AccountId>()
            .output::<Option<ArtistDataOutput>, false>()
            .ignore_error_code()
            .call(&account_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct ArtistDataOutput {
    pub owner: AccountId,
    pub registered_at: BlockNumber,
    pub verified_at: Option<BlockNumber>,
    pub main_name: Vec<u8>,
    pub alias: Option<Vec<u8>>,
    pub genres: Vec<MusicGenre>,
    pub description: Option<Hash>,
    pub assets: Vec<Hash>,
    pub contracts: Vec<AccountId>,
}
