use rand;
use hex::Hex;
use std::fmt;
use crypto::ed25519;
use std::ops::Deref;

pub fn keypair() -> (PublicKey, SecretKey) {
    let (s, p) = ed25519::keypair(&rand::random::<[u8; 32]>());
    (PublicKey(p), SecretKey(s))
}

#[derive(Clone, PartialEq)]
pub struct PublicKey(pub [u8; 32]);

impl PublicKey {
    pub fn new(data: &[u8]) -> PublicKey {
        let mut key = [0u8; 32];
        key.clone_from_slice(data);
        PublicKey(key)
    }
}

impl fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PublicKey({:x}...{:x})",
            Hex(&self.0[0..4]),
            Hex(&self.0[28..32])
        )
    }
}
impl fmt::LowerHex for PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", Hex(&self.0))
    }
}

impl Deref for PublicKey {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

pub struct SecretKey([u8; 64]);

impl SecretKey {
    pub fn exchange(&self, other: &PublicKey) -> SharedKey {
        SharedKey(ed25519::exchange(&other, &self))
    }
}

impl fmt::Debug for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SecretKey({:x}...{:x})",
            Hex(&self.0[0..4]),
            Hex(&self.0[60..64])
        )
    }
}
impl fmt::LowerHex for SecretKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", Hex(&self.0))
    }
}

impl Deref for SecretKey {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

pub struct SharedKey([u8; 32]);

impl fmt::Debug for SharedKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SharedKey({:x}...{:x})",
            Hex(&self.0[0..4]),
            Hex(&self.0[28..32])
        )
    }
}
impl fmt::LowerHex for SharedKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", Hex(&self.0))
    }
}
impl Deref for SharedKey {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}
//
// //
// //     pub fn sign(&self, message: &[u8]) -> SignedMessage {
// //         if let Some(ref secret) = self.secret {
// //             SignedMessage {
// //                 data: message.to_vec(),
// //                 signature: ed25519::signature(message, &secret),
// //             }
// //         } else {
// //             panic!("Missing private key");
// //         }
// //     }
// // }
//
// pub fn sample() {
//     // Generate a pair of keypairs.
//     let alice = Peer::generate();
//     println!("alice: {:?}", alice);
//     let bob = Peer::generate();
//     println!("bob: {:?}", bob);
//
//     // Extract external versions that don't know private key
//     let alice_ext = alice.externalize();
//     println!("alice_ext: {:?}", alice_ext);
//     let bob_ext = bob.externalize();
//     println!("bob_ext: {:?}", bob_ext);
//
//     // Calculate shared keys from both points of view
//     let shared = bob.exchange(&alice_ext);
//     println!("{:?}", shared);
//     let shared2 = alice.exchange(&bob_ext);
//     println!("{:?}", shared2);
//
//     // Sign a message
//     let signed = bob.sign(b"Hello World");
//     println!("{:?}", signed);
//     // Verify using correct identity
//     let message = signed.verify(&bob_ext);
//     println!("{:?}", message);
//     // Attempt verify using wrong identity
//     let message = signed.verify(&alice_ext);
//     println!("{:?}", message);
// }
