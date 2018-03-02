extern crate crypto;
extern crate rand;

mod hex;
mod peer;
// mod hash;

use peer::{keypair, PublicKey, SecretKey};

use crypto::chacha20poly1305::ChaCha20Poly1305;
use hex::Hex;
use crypto::aead::{AeadDecryptor, AeadEncryptor};

// output vec contains:
//   from: 32 bytes public key
//   to: 32 bytes public key
//   nonce: 8 random bytes
//   tag: 16 bytes authenticationt tag
//   encrypted payload: n bytes
fn make_parcel(sender: (&PublicKey, &SecretKey), receiver: &PublicKey, message: &[u8]) -> Vec<u8> {
    let (sender, sender_secret) = sender;
    let mut parcel = Vec::new();
    parcel.extend_from_slice(&sender);
    parcel.extend_from_slice(&receiver);
    let nonce = rand::random::<[u8; 8]>();
    let shared = sender_secret.exchange(&receiver);
    let mut channel = ChaCha20Poly1305::new(&shared, &nonce, &parcel);
    parcel.extend_from_slice(&nonce);
    let mut output = vec![0; message.len()];
    let mut tag = [0u8; 16];
    channel.encrypt(message, &mut output, &mut tag);
    parcel.extend_from_slice(&tag);
    parcel.extend_from_slice(&output);
    parcel
}

fn check_parcel(receiver: (&PublicKey, &SecretKey), parcel: &[u8]) -> Option<(PublicKey, Vec<u8>)> {
    if parcel.len() < (32 + 32 + 8 + 16) {
        return None;
    }
    let (receiver, receiver_secret) = receiver;
    let sender = PublicKey::new(&parcel[0..32]);
    let address: &[u8] = &parcel[32..64];
    let header: &[u8] = &parcel[0..64];
    let nonce: &[u8] = &parcel[64..72];
    let tag: &[u8] = &parcel[72..88];
    let payload: &[u8] = &parcel[88..];
    if &receiver as &[u8] != address {
        return None;
    }

    let shared = receiver_secret.exchange(&sender);
    let mut channel = ChaCha20Poly1305::new(&shared, nonce, header);
    let mut output = vec![0; payload.len()];
    if channel.decrypt(payload, &mut output, tag) {
        Some((sender, output))
    } else {
        None
    }
}

fn main() {
    let (alice, alice_secret) = keypair();
    let (bob, bob_secret) = keypair();

    let parcel = make_parcel((&alice, &alice_secret), &bob, b"Hello World");

    println!("{:x}", Hex(&parcel));

    let res = check_parcel((&bob, &bob_secret), &parcel);
    println!("{:?}", res);
}
