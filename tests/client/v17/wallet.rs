/// Requires `Client` to be in scope and to implement `createwallet`.
#[macro_export]
macro_rules! impl_test_v17__createwallet {
    () => {
        #[test]
        pub fn create_wallet() {
            // Implicitly tests createwalled because we create the default wallet.
            let _ = bitcoind_with_default_wallet();
        }
    };
}

/// Requires `Client` to be in scope and to implement `loadwallet`.
#[macro_export]
macro_rules! impl_test_v17__loadwallet {
    () => {
        #[test]
        fn load_wallet() {
            // Implicitly test loadwalled because we load the default wallet.
            let _ = bitcoind_with_default_wallet();
        }
    };
}

/// Requires `Client` to be in scope and to implement `unloadwallet`.
#[macro_export]
macro_rules! impl_test_v17__unloadwallet {
    () => {
        #[test]
        fn unload_wallet() {
            let bitcoind = bitcoind_no_wallet();
            let wallet = format!("wallet-{}", rand::random::<u32>()).to_string();
            bitcoind.client.create_wallet(&wallet).expect("failed to create wallet");
            let _ = bitcoind.client.unload_wallet(&wallet).expect("unloadwallet");
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_new_address`.
#[macro_export]
macro_rules! impl_test_v17__getnewaddress {
    () => {
        #[test]
        fn get_new_address() {
            let bitcoind = bitcoind_with_default_wallet();
            let _ = bitcoind.client.get_new_address().expect("getnewaddress");
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_balance`.
#[macro_export]
macro_rules! impl_test_v17__getbalance {
    () => {
        #[test]
        fn get_balance() {
            let bitcoind = bitcoind_with_default_wallet();
            let _ = bitcoind.client.get_balance().expect("getbalance");
        }
    };
}

/// Requires `Client` to be in scope and to implement `send_to_address`.
#[macro_export]
macro_rules! impl_test_v17__sendtoaddress {
    () => {
        #[test]
        fn send_to_address() {
            use bitcoin::Amount;

            let bitcoind = bitcoind_with_default_wallet();
            let address = bitcoind.client.new_address().expect("failed to create new address");
            let _ = bitcoind.client.generate_to_address(101, &address).expect("generatetoaddress");

            let balance = bitcoind.client.get_balance().expect("getbalance");
            dbg!(balance);

            let _ = bitcoind
                .client
                .send_to_address(&address, Amount::from_sat(10_000))
                .expect("sendtoaddress");
        }
    };
}
