// SPDX-License-Identifier: CC0-1.0

//! Tests for methods found under the `== Wallet ==` section of the API docs.

#[cfg(feature = "TODO")]
use bitcoin::address::{Address, NetworkChecked};
#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
use bitcoin::Amount;
use integration_test::{Node, NodeExt as _, Wallet};
#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
use node::AddressType;

#[test]
#[cfg(feature = "TODO")]
pub fn add_multisig_address() {
    let nrequired = 1; // 1-of-2 multisig.

    let add1: Address<NetworkChecked> =
        "32iVBEu4dxkUQk9dJbZUiBiQdmypcEyJRf".parse::<Address<_>>().unwrap().assume_checked();
    let add2: Address<NetworkChecked> =
        "132F25rTsvBdp9JzLLBHP5mvGY66i1xdiM".parse::<Address<_>>().unwrap().assume_checked();

    let node = Node::with_wallet(Wallet::Default, &[]);
    let json = node
        .client
        .add_multisig_address_with_addresses(nrequired, vec![add1, add2])
        .expect("addmultisigaddress");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
pub fn bump_fee() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to create new address");
    let _ = node.client.generate_to_address(101, &address).expect("generatetoaddress");

    let txid = node
        .client
        .send_to_address_rbf(&address, Amount::from_sat(10_000))
        .expect("sendtoaddress")
        .txid()
        .unwrap();

    let json = node.client.bump_fee(txid).expect("bumpfee");
    assert!(json.into_model().is_ok());
}

#[test]
pub fn create_wallet() {
    // Implicitly tests `createwallet` because we create the default wallet.
    let _ = Node::with_wallet(Wallet::Default, &[]);
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
pub fn dump_priv_key() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to create new address");
    let json = node.client.dump_priv_key(&address).expect("dumpprivkey");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
pub fn dump_wallet() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let out = integration_test::random_tmp_file();
    let json = node.client.dump_wallet(&out).expect("dumpwallet");
    let _ = json.into_model();
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
pub fn get_addresses_by_label() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let label = "some-label";
    let addr = node.client.new_address_with_label(label).expect("failed to get new address");
    let json = node.client.get_addresses_by_label(label).expect("getaddressesbylabel");
    let map = json.into_model().expect("failed to convert to model").0;
    assert!(!map.is_empty());
    assert!(map.get(&addr).is_some());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
// TODO: Consider testing a few different address types.
#[cfg(feature = "TODO")]
pub fn get_address_info() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let address = node.client.new_address().expect("failed to create new address");
    let json = node.client.get_address_info(&address).expect("getaddressinfo");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_balance() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let json = node.client.get_balance().expect("getbalance");
    assert!(json.into_model().is_ok());

    node.fund_wallet();
    let json = node.client.get_balance().expect("getbalance");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
#[cfg(feature = "v19")]
fn get_balances() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let json = node.client.get_balances().expect("getbalances");
    let model = json.into_model().expect("into_model");
    // TODO: Do more fine grained testing.
    assert!(model.mine.trusted > Amount::ZERO);
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_new_address() {
    let node = Node::with_wallet(Wallet::Default, &[]);

    let _ = node.client.new_address().expect("getnewaddress");

    // Test the helper as well just for good measure.
    let _ = node.client.new_address().unwrap();

    // Exhaustively test address types with helper.
    let _ = node.client.new_address_with_type(AddressType::Legacy).unwrap();
    let _ = node.client.new_address_with_type(AddressType::P2shSegwit).unwrap();
    let _ = node.client.new_address_with_type(AddressType::Bech32).unwrap();
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_raw_change_address() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    let json = node.client.get_raw_change_address().expect("getrawchangeaddress");
    assert!(json.into_model().is_ok());
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_received_by_address() {
    let amount = Amount::from_sat(10_000);

    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let _txid =
        node.client.send_to_address(&address, amount).expect("sendtoaddress").txid().unwrap();
    node.mine_a_block();

    let json = node.client.get_received_by_address(&address).expect("getreceivedbyaddress");
    let model = json.into_model().expect("into_model failed");
    assert_eq!(model.0, amount);
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn get_transaction() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let txid = node
        .client
        .send_to_address(&address, Amount::from_sat(10_000))
        .expect("sendtoaddress")
        .txid()
        .unwrap();

    let json = node.client.get_transaction(txid).expect("gettransaction");
    assert!(json.into_model().is_ok());
}

#[test]
fn load_wallet() {
    create_load_unload_wallet();
}

#[test]
fn unload_wallet() {
    create_load_unload_wallet();
}

fn create_load_unload_wallet() {
    let node = Node::with_wallet(Wallet::None, &[]);

    let wallet = format!("wallet-{}", rand::random::<u32>()).to_string();
    node.client.create_wallet(&wallet).expect("failed to create wallet");

    // Upto version 20 Core returns null for `unloadwallet`.
    #[cfg(any(feature = "v17", feature = "v18", feature = "v19", feature = "v20"))]
    let _ = node.client.unload_wallet(&wallet).expect("unloadwallet");

    // From version 21 Core returns warnings for `unloadwallet`.
    #[cfg(all(not(feature = "v17"), not(feature = "v18"), not(feature = "v19"), not(feature = "v20")))]
    {
        let json = node.client.unload_wallet(&wallet).expect("unloadwallet");
        let _ = json.into_model();
    }

    let json = node.client.load_wallet(&wallet).expect("loadwallet");
    let _ = json.into_model();
}

#[cfg(any(feature = "0_17_1", feature = "0_18_1"))]
#[test]
fn send_to_address() {
    let node = Node::with_wallet(Wallet::Default, &[]);
    node.fund_wallet();
    let address = node.client.new_address().expect("failed to create new address");

    let json =
        node.client.send_to_address(&address, Amount::from_sat(10_000)).expect("sendtddress");
    assert!(json.into_model().is_ok());
}
