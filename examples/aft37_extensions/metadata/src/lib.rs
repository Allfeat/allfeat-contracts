#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT37, AFT37Metadata)]
#[allfeat_contracts::contract]
pub mod my_aft37_metadata {
    use openbrush::traits::{Storage, String};

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        aft37: aft37::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn set_attribute(
            &mut self,
            id: Id,
            key: String,
            data: String,
        ) -> Result<(), AFT37Error> {
            metadata::Internal::_set_attribute(self, &id, &key, &data)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use allfeat_contracts::aft37::extensions::metadata::aft37metadata_external::AFT37Metadata;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn metadata_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_aft37_metadata", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let id = Id::U8(0);
            let attr = String::from("https://www.allfeat.network/");

            let attribute = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_attribute(id.clone(), attr.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(attribute, None);

            let set_attribute_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.set_attribute(
                        id.clone(),
                        attr.clone(),
                        String::from("https://www.allfeat.network/"),
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
            }
            .return_value();

            assert_eq!(set_attribute_tx, Ok(()));

            let attribute = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.get_attribute(id.clone(), attr.clone()));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert_eq!(
                attribute,
                Some(String::from("https://www.allfeat.network/"))
            );

            Ok(())
        }
    }
}
