#![cfg_attr(not(feature = "std"), no_std, no_main)]

// pub use my_aft22::*;
pub use openbrush::traits::{AccountId, Storage};

// we need to expand this struct before the contract macro is expanded
// that is why we declare it here for this example
#[ink::storage_item]
#[openbrush::accessors(HatedStorageAccessors)]
#[derive(Debug)]
pub struct HatedStorage {
    #[get]
    #[set]
    pub hated_account: AccountId,
}

#[allfeat_contracts::implementation(AFT22)]
#[allfeat_contracts::contract]
pub mod my_aft22 {
    use crate::*;
    use openbrush::traits::String;

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Contract {
        #[storage_field]
        aft22: aft22::Data,
        #[storage_field]
        hated_storage: HatedStorage,
    }

    #[overrider(aft22::AFT22Transfer)]
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), AFT22Error> {
        if _to == Some(&self.hated_storage.hated_account) {
            return Err(AFT22Error::Custom(String::from("I hate this account!")));
        }
        Ok(())
    }

    impl HatedStorageAccessors for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut instance = Self {
                aft22: Default::default(),
                hated_storage: HatedStorage {
                    hated_account: [255; 32].into(),
                },
            };

            Internal::_mint_to(&mut instance, Self::env().caller(), total_supply)
                .expect("Should mint");

            instance
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use super::*;
        use crate::hatedstorageaccessors_external::HatedStorageAccessors;
        use allfeat_contracts::aft22::aft22_external::AFT22;
        use ink_e2e::build_message;
        use test_helpers::{address_of, balance_of};

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn assigns_initial_balance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_aft22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.balance_of(address_of!(alice)));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), 100));

            Ok(())
        }

        #[ink_e2e::test]
        async fn transfer_adds_amount_to_destination_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_aft22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 50, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_alice = balance_of!(client, address, alice);

            let balance_of_bob = balance_of!(client, address, bob);

            assert_eq!(balance_of_bob, 50, "Bob should have 50 tokens");
            assert_eq!(balance_of_alice, 50, "Alice should have 50 tokens");

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_above_the_amount(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_aft22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 101, vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(
                result.return_value(),
                Err(AFT22Error::InsufficientBalance)
            ));

            Ok(())
        }

        #[ink_e2e::test]
        async fn cannot_transfer_to_hated_account(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {
            let constructor = ContractRef::new(100);
            let address = client
                .instantiate("my_aft22", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("transfer failed")
            };

            assert!(matches!(result.return_value(), Ok(())));

            let balance_of_bob = balance_of!(client, address, bob);

            assert!(matches!(balance_of_bob, 10));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.set_hated_account(address_of!(bob)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("set_hated_account failed")
            };

            assert!(matches!(result.return_value(), ()));

            let result = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.transfer(address_of!(bob), 10, vec![]));
                client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
            };

            assert!(matches!(result.return_value(), Err(AFT22Error::Custom(_))));

            let balance_of_bob = balance_of!(client, address, bob);

            assert!(matches!(balance_of_bob, 10));

            Ok(())
        }
    }
}
