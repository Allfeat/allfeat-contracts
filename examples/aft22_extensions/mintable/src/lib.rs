#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT22, AFT22Mintable)]
#[allfeat_contracts::contract]
pub mod my_aft22_mintable {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        aft22: aft22::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self::default();

            aft22::Internal::_mint_to(&mut instance, Self::env().caller(), total_supply)
                .expect("Should mint");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use allfeat_contracts::aft22::{
            aft22_external::AFT22, extensions::mintable::aft22mintable_external::AFT22Mintable,
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        use test_helpers::{address_of, balance_of};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(1000);
            let address = client
                .instantiate("my_aft22_mintable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, alice), 1000));

            Ok(())
        }

        #[ink_e2e::test]
        async fn minting_requested_amount(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(1000);
            let address = client
                .instantiate("my_aft22_mintable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(
                matches!(balance_of!(client, address, bob), 0),
                "Bob's balance should be 0"
            );

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), 1000));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(
                matches!(mint_tx.return_value(), Ok(())),
                "Minting should be successful"
            );

            assert!(
                matches!(balance_of!(client, address, bob), 1000),
                "Bob's balance should be 1000"
            );

            Ok(())
        }

        #[ink_e2e::test]
        async fn increases_total_supply_after_minting(
            client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(0);
            let address = client
                .instantiate("my_aft22_mintable", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(total_supply, 0), "Total supply should be 0");

            let mint_tx = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(bob), 1000));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(
                matches!(mint_tx.return_value(), Ok(())),
                "Minting should be successful"
            );

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(total_supply, 1000), "Total supply should be 1000");

            Ok(())
        }
    }
}
