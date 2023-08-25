#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT37, AFT37Mintable)]
#[allfeat_contracts::contract]
pub mod my_aft37_mintable {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        aft37: aft37::Data,
    }

    impl Contract {
        /// contract constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use allfeat_contracts::aft37::{
            aft37_external::AFT37, extensions::mintable::aft37mintable_external::AFT37Mintable,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e:: dbuild_message;

        use test_helpers::{address_of, balance_of_37};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn mint_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_aft37_mintable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 2;

            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_1.clone())),
                0
            );
            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_1.clone())),
                0
            );
            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_2.clone())),
                0
            );
            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_2.clone())),
                0
            );

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(address_of!(alice), vec![(token_1.clone(), amount_1)])
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(address_of!(alice), vec![(token_2.clone(), amount_2)])
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(bob),
                        vec![(token_1.clone(), amount_1), (token_2.clone(), amount_2)],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_1.clone())),
                amount_1
            );
            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_1.clone())),
                amount_1
            );
            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_2.clone())),
                amount_2
            );
            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_2.clone())),
                amount_2
            );

            Ok(())
        }
    }
}
