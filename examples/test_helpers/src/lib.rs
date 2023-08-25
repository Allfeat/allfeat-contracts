#[macro_export]
macro_rules! address_of {
    ($account:ident) => {
        ink::primitives::AccountId::from(ink_e2e::$account().public_key().0)
    };
}

#[macro_export]
macro_rules! balance_of {
    ($client:ident, $address:ident, $account:ident) => {{
        let _msg = build_message::<ContractRef>($address.clone())
            .call(|contract| contract.balance_of(address_of!($account)));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! owner_of {
    ($client:ident, $address:ident, $id:expr) => {{
        let _msg =
            build_message::<ContractRef>($address.clone()).call(|contract| contract.owner_of($id));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}

#[macro_export]
macro_rules! balance_of_37 {
    ($client:ident, $address:ident, $account:ident, $token:expr) => {{
        let _msg = build_message::<ContractRef>($address.clone())
            .call(|contract| contract.balance_of(address_of!($account), $token));
        $client
            .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
            .await
            .return_value()
    }};
}
