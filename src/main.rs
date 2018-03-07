extern crate crypto;
extern crate rand;
use std::time::{SystemTime, UNIX_EPOCH};

// #[macro_use]
// extern crate bincode;
// extern crate serde_derive;
// use bincode::{deserialize, serialize, Infinite};

mod hex;
mod peer;
// mod hash;
use std::collections::BTreeMap;

use peer::{keypair, PublicKey, SecretKey};

use crypto::chacha20poly1305::ChaCha20Poly1305;
use hex::Hex;
use crypto::aead::{AeadDecryptor, AeadEncryptor};

struct Parcel {
    sender: PublicKey,
    receiver: PublicKey,
    timestamp: u32,
    nonce: [u8; 8],
    tag: [u8; 16],
    payload: Vec<u8>,
}

impl Parcel {
    fn new(sender: (&PublicKey, &SecretKey), receiver: &PublicKey, message: &[u8]) -> Self {
        let (sender, sender_secret) = sender;
        let nonce = rand::random::<[u8; 8]>();
        let shared = sender_secret.exchange(&receiver);
        let mut channel = ChaCha20Poly1305::new(&shared, &nonce, &parcel);
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let mut payload = vec![0; message.len()];
        let mut tag = [0u8; 16];
        channel.encrypt(message, &mut payload, &mut tag);
        Parcel {
            sender,
            receiver,
            nonce,
            timestamp,
            tag,
            payload,
        }
    }

    fn check(
        &self,
        recent: &mut BTreeMap<[u8; 8], u32>,
        receiver: (&PublicKey, &SecretKey),
    ) -> (PublicKey, Vec<u8>) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        if now > now + 60 || now < now - 60 {
            // If the timestamp is stale, ignore the message.
            return None;
        }
        if let Some(time) = recent.get(nonce) {
            if *time == timestamp {
                // If the message is a replay, ignore it.
                return None;
            }
        }
        let mut copy = [0u8; 8];
        copy.copy_from_slice(nonce);
        recent.insert(copy, timestamp);
        if &receiver as &[u8] != address {
            // If it's not addressed to us, ignore it.
            return None;
        }
        let shared = receiver_secret.exchange(&sender);
        let mut channel = ChaCha20Poly1305::new(&shared, nonce, header);
        let mut output = vec![0; payload.len()];
        if channel.decrypt(payload, &mut output, tag) {
            Some((sender, output))
        } else {
            // Ignore malformed messages.
            None
        }
    }
}

fn main() {
    let (alice, alice_secret) = keypair();
    let (bob, bob_secret) = keypair();

    let parcel = Parcel::new((&alice, &alice_secret), &bob, b"Hello World");

    println!("{:?}", parcel);

    let mut recent = BTreeMap::new();

    let res = parcel.check(&mut recent, (&bob, &bob_secret));
    println!("{:?}", res);
    let res2 = parcel.check(&mut recent, (&bob, &bob_secret));
    println!("{:?}", res2);
}
