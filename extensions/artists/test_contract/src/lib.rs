#![cfg_attr(not(feature = "std"), no_std, no_main)]

use allfeat_contracts_extensions_artists::extension::ArtistExtension;

#[ink::contract]
mod artists_extension_wrapper {
    use super::*;
    use allfeat_contracts_extensions_artists::types::{ArtistData, CandidateData};

    #[ink(storage)]
    pub struct MyStorage;

    impl MyStorage {
        #[ink(constructor)]
        pub fn new() -> Self {
            MyStorage {}
        }

        // Constants
        #[ink(message)]
        pub fn creation_deposit_amount(&self) -> Balance {
            ArtistExtension::creation_deposit_amount()
        }
        #[ink(message)]
        pub fn name_max_length(&self) -> u32 {
            ArtistExtension::name_max_length()
        }

        // Chain State Queries
        #[ink(message)]
        pub fn artists(&self, account_id: AccountId) -> Option<ArtistData<BlockNumber>> {
            ArtistExtension::artists(account_id)
        }
        #[ink(message)]
        pub fn candidates(&self, account_id: AccountId) -> Option<CandidateData<BlockNumber>> {
            ArtistExtension::candidates(account_id)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use super::*;
        use ink_e2e::build_message;
        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn creation_deposit_amount_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = MyStorageRef::new();
            let address = client
                .instantiate(
                    "artists_extension_wrapper",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<MyStorageRef>(address.clone())
                    .call(|contract| contract.creation_deposit_amount());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), 50_000_000_000u128));
            Ok(())
        }
    }
}
