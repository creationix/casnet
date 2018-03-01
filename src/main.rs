extern crate crypto;
extern crate rand;

mod hex;

use crypto::ed25519;
use hex::Hex;

fn main() {
    let seed = rand::random::<[u8; 32]>();
    let (private, public) = ed25519::keypair(&seed);
    println!("{:x}", Hex(&seed));
    println!("{:x}", Hex(&public));
    println!("{:x}", Hex(&private));
}
