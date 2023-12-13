#![cfg_attr(not(feature = "std"), no_std, no_main)]

use allfeat_contracts_extensions_artists::ArtistDataOutput;
use allfeat_contracts_extensions_artists::ArtistExtension;

#[ink::contract]
mod artists_extension_wrapper {
    use super::*;

    #[ink(storage)]
    pub struct MyStorage;

    impl MyStorage {
        #[ink(constructor)]
        pub fn new() -> Self {
            MyStorage {}
        }

        // Chain State Queries
        #[ink(message)]
        pub fn get_artist(&self, account_id: AccountId) -> Option<ArtistDataOutput> {
            ArtistExtension::artist(account_id)
        }
    }
}
