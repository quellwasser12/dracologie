extern crate hex;
extern crate bitcoin_hashes;


use bitcoin_hashes::{hash160, Hash};
use hdwallet::secp256k1::{Secp256k1, SecretKey, PublicKey};

fn to_hash160(bytes: &[u8]) -> [u8;20] {
    return hash160::Hash::hash(bytes).into_inner();
}


pub fn keyinfo(key: String) {
    let secp = Secp256k1::new();
    let seed = hex::decode(key).expect("Decode failed");

    let private_key = SecretKey::from_slice(&seed).unwrap();
    let public_key = PublicKey::from_secret_key(&secp, &private_key);


    println!("Private Key: {:?}", private_key);
    println!("Public Key: {}", hex::encode(public_key.serialize_uncompressed()));

    let h = to_hash160(&public_key.serialize_uncompressed());
    println!("Public Key HASH160: {}", hex::encode(h));
    println!("Compressed Public Key: {}", hex::encode(public_key.serialize()));

    let h = to_hash160(&public_key.serialize());
    println!("Compressed Public Key HASH160: {}", hex::encode(h));
}
