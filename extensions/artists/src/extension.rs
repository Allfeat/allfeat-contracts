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

use crate::types::{ArtistData, CandidateData};
use ink_crate::env::{DefaultEnvironment, Environment};

pub type AccountId = <DefaultEnvironment as Environment>::AccountId;
pub type Balance = <DefaultEnvironment as Environment>::Balance;
pub type BlockNumber = <DefaultEnvironment as Environment>::BlockNumber;

/// Implementation of all functions contained in the pallet_artists available to ink! contracts.
pub struct ArtistExtension;

impl ArtistExtension {
    // Getters constants
    pub fn creation_deposit_amount() -> Balance {
        ::ink_crate::env::chain_extension::ChainExtensionMethod::build(0001u32)
            .input::<()>()
            .output::<Balance, false>()
            .ignore_error_code()
            .call(&())
    }
    pub fn name_max_length() -> u32 {
        ::ink_crate::env::chain_extension::ChainExtensionMethod::build(0002u32)
            .input::<()>()
            .output::<u32, false>()
            .ignore_error_code()
            .call(&())
    }

    // Chain State Queries
    pub fn artists(account_id: AccountId) -> Option<ArtistData<BlockNumber>> {
        ::ink_crate::env::chain_extension::ChainExtensionMethod::build(0051u32)
            .input::<AccountId>()
            .output::<Option<ArtistData<BlockNumber>>, false>()
            .ignore_error_code()
            .call(&account_id)
    }
    pub fn candidates(candidate_id: AccountId) -> Option<CandidateData<BlockNumber>> {
        ::ink_crate::env::chain_extension::ChainExtensionMethod::build(0052u32)
            .input::<AccountId>()
            .output::<Option<CandidateData<BlockNumber>>, false>()
            .ignore_error_code()
            .call(&candidate_id)
    }
}
