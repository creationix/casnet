extern crate crypto;
extern crate rand;

mod hex;

use hex::Hex;
use crypto::ed25519;
use crypto::blake2b::Blake2b;

fn main() {
    let seed = rand::random::<[u8; 32]>();
    let (private, public) = ed25519::keypair(&seed);
    println!("{:x}", Hex(&seed));
    println!("{:x}", Hex(&public));
    println!("{:x}", Hex(&private));
    let mut hash = [0u8; 32];
    Blake2b::blake2b(&mut hash, b"Hello", b"");
    println!("{:x}", Hex(&hash));
}
