#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT37, AFT37Batch)]
#[allfeat_contracts::contract]
pub mod my_aft37_batch {
    use openbrush::traits::Storage;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Contract {
        #[storage_field]
        aft37: aft37::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self::default()
        }

        #[ink(message)]
        pub fn mint(
            &mut self,
            to: AccountId,
            ids_amounts: Vec<(Id, Balance)>,
        ) -> Result<(), AFT37Error> {
            aft37::Internal::_mint_to(self, to, ids_amounts)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use allfeat_contracts::aft37::{
            aft37_external::AFT37, extensions::batch::aft37batch_external::AFT37Batch,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{address_of, balance_of_37};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn batch_transfer_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_aft37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![
                            (token_1.clone(), amount_1.clone()),
                            (token_2.clone(), amount_2.clone()),
                        ],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let batch_transfer_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer(
                        address_of!(bob),
                        vec![
                            (token_1.clone(), amount_1.clone()),
                            (token_2.clone(), amount_2.clone()),
                        ],
                        vec![],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(batch_transfer_tx, Ok(()));

            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_1.clone())),
                amount_1
            );
            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_2.clone())),
                amount_2
            );
            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_1.clone())),
                0
            );
            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_2.clone())),
                0
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn batch_transfer_from_should_work(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_aft37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![
                            (token_1.clone(), amount_1.clone()),
                            (token_2.clone(), amount_2.clone()),
                        ],
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
                balance_of_37!(client, address, alice, Some(token_2.clone())),
                amount_2
            );
            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_1.clone())),
                0
            );
            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_2.clone())),
                0
            );

            let batch_transfer_from_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer_from(
                        address_of!(alice),
                        address_of!(bob),
                        vec![
                            (token_1.clone(), amount_1.clone()),
                            (token_2.clone(), amount_2.clone()),
                        ],
                        vec![],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(batch_transfer_from_tx, Ok(()));

            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_1.clone())),
                amount_1
            );
            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_2.clone())),
                amount_2
            );
            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_1.clone())),
                0
            );
            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_2.clone())),
                0
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn batch_transfer_from_with_no_approve_should_fail(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_aft37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![
                            (token_1.clone(), amount_1.clone()),
                            (token_2.clone(), amount_2.clone()),
                        ],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let batch_transfer_from_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer_from(
                        address_of!(bob),
                        address_of!(alice),
                        vec![
                            (token_1.clone(), amount_1.clone()),
                            (token_2.clone(), amount_2.clone()),
                        ],
                        vec![],
                    )
                });
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(batch_transfer_from_tx, Err(_)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn batch_transfer_from_with_approve_should_work(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new();
            let address = client
                .instantiate("my_aft37_batch", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let token_1 = Id::U8(0);
            let token_2 = Id::U8(1);

            let amount_1 = 1;
            let amount_2 = 20;

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.mint(
                        address_of!(alice),
                        vec![
                            (token_1.clone(), amount_1.clone()),
                            (token_2.clone(), amount_2.clone()),
                        ],
                    )
                });
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert_eq!(mint_tx, Ok(()));

            let approve_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.approve(address_of!(bob), None, 1));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("approve failed")
            }
            .return_value();

            assert_eq!(approve_tx, Ok(()));

            let batch_transfer_from_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| {
                    contract.batch_transfer_from(
                        address_of!(alice),
                        address_of!(bob),
                        vec![
                            (token_1.clone(), amount_1.clone()),
                            (token_2.clone(), amount_2.clone()),
                        ],
                        vec![],
                    )
                });
                client
                    .call(&ink_e2e::bob(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            }
            .return_value();

            assert_eq!(batch_transfer_from_tx, Ok(()));

            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_1.clone())),
                amount_1
            );
            assert_eq!(
                balance_of_37!(client, address, bob, Some(token_2.clone())),
                amount_2
            );
            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_1.clone())),
                0
            );
            assert_eq!(
                balance_of_37!(client, address, alice, Some(token_2.clone())),
                0
            );

            Ok(())
        }
    }
}
