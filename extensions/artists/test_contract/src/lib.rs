#![cfg_attr(not(feature = "std"), no_std, no_main)]

use allfeat_contracts_extensions_artists::ArtistDataOutput;
use allfeat_contracts_extensions_artists::ArtistExtension;
use ink::prelude::string::String;

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
        pub fn artist_by_id(&self, account_id: AccountId) -> Option<ArtistDataOutput> {
            ArtistExtension::artists_by_id(account_id)
        }
        #[ink(message)]
        pub fn artist_by_name(&self, name: String) -> Option<ArtistDataOutput> {
            ArtistExtension::artists_by_name(name)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        //use super::*;
        //use ink_e2e::build_message;
        //type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;
    }
}
