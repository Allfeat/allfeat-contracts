#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT37, AFT37URIStorage, AFT37Mintable)]
#[allfeat_contracts::contract]
pub mod my_aft37_uri_storage {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        aft37: aft37::Data,
        #[storage_field]
        uris: uri_storage::Data,
    }

    impl Contract {
        /// A constructor which set the base uri of the collection.
        #[ink(constructor)]
        pub fn new(base_uri: Option<URI>) -> Self {
            let mut instance = Default::default();
            uri_storage::Internal::_set_base_uri(&mut instance, base_uri);
            instance
        }
        #[ink(message)]
        pub fn set_token_uri(&mut self, token_id: Id, token_uri: URI) -> Result<(), AFT37Error> {
            uri_storage::Internal::_set_token_uri(self, token_id, token_uri)?;
            Ok(())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        use allfeat_contracts::aft37::{
            extensions::mintable::aft37mintable_external::AFT37Mintable,
            extensions::uri_storage::aft37uristorage_external::AFT37URIStorage,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        use test_helpers::address_of;

        #[ink_e2e::test]
        async fn base_uri_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let default_base_uri = Some(String::from("https://allfeat.network/"));

            let constructor = ContractRef::new(default_base_uri.clone());
            let address = client
                .instantiate(
                    "my_aft37_uri_storage",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let result_base_uri = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.base_uri());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(result_base_uri, default_base_uri);

            Ok(())
        }

        #[ink_e2e::test]
        async fn only_token_uri_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let id = Id::U8(0);
            let uri = URI::from("aft37_1");

            // testing without base uri
            let constructor = ContractRef::new(None);
            let address = client
                .instantiate(
                    "my_aft37_uri_storage",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let result_set_token_uri_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_token_uri(id.clone(), uri.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            // Can't set cause token isn't minted
            assert_eq!(result_set_token_uri_1, Err(AFT37Error::TokenNotExists));

            let result_token_uri_1 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_uri(id.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            // Also cant retrieve
            assert_eq!(result_token_uri_1, Err(AFT37Error::TokenNotExists));

            let _mint = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), vec![(id.clone(), 1)]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            let result_set_token_uri_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_token_uri(id.clone(), uri.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("set call failed")
            }
            .return_value();

            // should have set now
            assert_eq!(result_set_token_uri_2, Ok(()));

            let result_token_uri_2 = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_uri(id.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(result_token_uri_2, Ok(Some(uri)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn base_and_token_uri_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let id = Id::U8(0);
            let base = Some(URI::from("https://allfeat.network/"));
            let uri = URI::from("aft37_1");

            // testing without base uri
            let constructor = ContractRef::new(base.clone());
            let address = client
                .instantiate(
                    "my_aft37_uri_storage",
                    &ink_e2e::alice(),
                    constructor,
                    0,
                    None,
                )
                .await
                .expect("instantiate failed")
                .account_id;

            let _mint = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), vec![(id.clone(), 1)]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            let result_set_token_uri = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_token_uri(id.clone(), uri.clone()));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("set call failed")
            }
            .return_value();

            // should have set now
            assert_eq!(result_set_token_uri, Ok(()));

            let result_token_uri = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.token_uri(id.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(result_token_uri, Ok(Some(base.unwrap() + &uri)));

            Ok(())
        }
    }
}
