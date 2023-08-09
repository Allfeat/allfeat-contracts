#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT34, AFT34URIStorage, AFT34Mintable)]
#[allfeat_contracts::contract]
pub mod my_aft34_uri_storage {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        aft34: aft34::Data,
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
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        use allfeat_contracts::aft34::{
            aft34_external::AFT34,
            extensions::uri_storage::aft34uristorage_external::AFT34URIStorage,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message};

        use test_helpers::balance_of;

        #[ink_e2e::test]
        async fn base_uri_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let default_base_uri = Some(String::from("https://allfeat.network/"));

            let constructor = ContractRef::new(default_base_uri.clone());
            let address = client
                .instantiate(
                    "my_aft34_uri_storage",
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
    }
}
