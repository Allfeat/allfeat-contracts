#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[allfeat_contracts::implementation(AFT22, Flashmint)]
#[allfeat_contracts::contract]
pub mod my_aft22_flashmint {
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        aft22: aft22::Data,
    }

    /// Override `get_fee` function to add 1% fee to the borrowed `amount`
    #[overrider(flashmint::Internal)]
    fn _get_fee(&self, amount: Balance) -> Balance {
        amount / 100
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
}
