extern crate crypto;
extern crate rand;

mod hex;

use hex::Hex;
use crypto::ed25519;
use crypto::blake2b::Blake2b;

fn main() {
    // Generate a pair of keypairs.
    let bob = ed25519::keypair(&rand::random::<[u8; 32]>());
    println!("{:x}", Hex(&bob.0));
    println!("{:x}", Hex(&bob.1));
    let alice = ed25519::keypair(&rand::random::<[u8; 32]>());
    println!("{:x}", Hex(&alice.0));
    println!("{:x}", Hex(&alice.1));

    // Calculated shared keys using public keys
    let shared1 = ed25519::exchange(&bob.1, &alice.0);
    let shared2 = ed25519::exchange(&alice.1, &bob.0);
    println!("{:x}", Hex(&shared1));
    println!("{:x}", Hex(&shared2));

    // Take the blake2b hash of a value
    let mut hash = [0u8; 32];
    Blake2b::blake2b(&mut hash, b"Hello", b"");
    println!("{:x}", Hex(&hash));
}
