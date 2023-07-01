use zcash_client_backend::address::UnifiedAddress;
use zcash_client_backend::encoding::encode_payment_address;
use zcash_primitives::zip339::Mnemonic;
use zcash_primitives::zip32::{ExtendedSpendingKey, ChildIndex};
use zcash_primitives::consensus::Network::MainNetwork;
use zcash_primitives::consensus::Parameters;
use orchard::keys::{SpendingKey, FullViewingKey, Scope};

fn main() {
    let seed = "badge bless baby bird anger wage memory extend word isolate equip faith";
    const PURPOSE: u32 = 32;
    const COIN_TYPE: u32 = 133;
    let account = 0;

    // let mnemonic = Mnemonic::generate(Count::Words12);
    let mnemonic = Mnemonic::from_phrase(seed).expect("phrase is valid");
    let phrase = mnemonic.phrase();
    println!("Mnemonic phrase: {}", phrase);

    let seed = mnemonic.to_seed("");

    let spending_key = ExtendedSpendingKey::master(&seed);
    let (_ , payment_address) = spending_key
        .derive_child(ChildIndex::Hardened(PURPOSE))
        .derive_child(ChildIndex::Hardened(COIN_TYPE))
        .derive_child(ChildIndex::Hardened(account))
        .default_address();
    let sapling_address = encode_payment_address(MainNetwork.hrp_sapling_payment_address(), &payment_address);
    println!("Zcash address:           {:?}", sapling_address);
    println!("Goal address (sapling):  {:?}", "zs1duqpcc2ql7zfjttdm2gpawe8t5ecek5k834u9vdg4mqhw7j8j39sgjy8xguvk2semyd4ujeyj28");

    let orchard_sk = SpendingKey::from_zip32_seed(&seed, COIN_TYPE, 0).expect("Failed to derive Orchard key from seed.");
    let fvk = FullViewingKey::from(&orchard_sk);
    let a = fvk.address_at(0u64, Scope::External);
    let orchard_address = UnifiedAddress::from_receivers(Some(a), None, None).unwrap();
    let orchard_address_str = orchard_address.encode(&MainNetwork);

    println!("Orchard address:         {:?}", orchard_address_str);
    println!("Goal address (orchard):  {:?}", "u1zpfqm4r0cc5ttvt4mft6nvyqe3uwsdcgx65s44sd3ar42rnkz7v9az0ez7dpyxvjcyj9x0sd89yy7635vn8fplwvg6vn4tr6wqpyxqaw");

    let combined_ua = UnifiedAddress::from_receivers(Some(a), Some(payment_address), None).unwrap();
    println!("Combined address:        {:?}", combined_ua.encode(&MainNetwork));
}
