#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT22, AFT22Capped, AFT22Mintable)]
#[allfeat_contracts::contract]
pub mod my_aft22_capped {
    use openbrush::traits::{Storage, String};

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        aft22: aft22::Data,
        #[storage_field]
        cap: capped::Data,
    }

    #[overrider(aft22::Internal)]
    fn _before_token_transfer(
        &mut self,
        from: Option<&AccountId>,
        _: Option<&AccountId>,
        amount: &Balance,
    ) -> Result<(), AFT22Error> {
        // `is_none` means that it is minting
        if from.is_none() && capped::Internal::_is_cap_exceeded(self, amount) {
            return Err(AFT22Error::Custom(String::from("Cap exceeded")));
        }
        Ok(())
    }

    impl Contract {
        /// Constructor which mints `initial_supply` of the token to sender
        /// Will set the token's cap to `cap`
        #[ink(constructor)]
        pub fn new(inital_supply: Balance, cap: Balance) -> Self {
            let mut instance = Self::default();

            assert!(capped::Internal::_init_cap(&mut instance, cap).is_ok());
            assert!(
                AFT22Mintable::mint(&mut instance, Self::env().caller(), inital_supply).is_ok()
            );

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use allfeat_contracts::aft22::{
            aft22_external::AFT22,
            extensions::{
                capped::aft22capped_external::AFT22Capped,
                mintable::aft22mintable_external::AFT22Mintable,
            },
        };

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::build_message;

        use test_helpers::{address_of, balance_of};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn new_works(client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(1000, 2000);
            let address = client
                .instantiate("my_aft22_capped", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, alice), 1000));

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(total_supply, 1000));

            let cap = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.cap());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(cap, 2000));

            Ok(())
        }

        #[ink_e2e::test]
        async fn can_mint_when_total_supply_is_lower_then_cap(
            client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(1000, 2000);
            let address = client
                .instantiate("my_aft22_capped", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, alice), 1000));

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(total_supply, 1000));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), 1000));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("mint failed")
            }
            .return_value();

            assert!(matches!(result, Ok(())));
            assert!(matches!(balance_of!(client, address, alice), 2000));

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(total_supply, 2000));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_mint_if_total_supply_will_exceed_the_cap(
            client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(1000, 2000);
            let address = client
                .instantiate("my_aft22_capped", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            assert!(matches!(balance_of!(client, address, alice), 1000));

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(total_supply, 1000));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.mint(address_of!(alice), 1001));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(result, Err(AFT22Error::Custom(_))));
            assert!(matches!(balance_of!(client, address, alice), 1000));

            let total_supply = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.total_supply());
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            }
            .return_value();

            assert!(matches!(total_supply, 1000));

            Ok(())
        }
    }
}
