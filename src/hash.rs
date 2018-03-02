use hex::Hex;
use std::fmt;
use crypto::blake2b::Blake2b;

pub struct Hash([u8; 32]);

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Hash({:x}...{:x})",
            Hex(&self.0[0..4]),
            Hex(&self.0[28..32])
        )
    }
}
impl fmt::LowerHex for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", Hex(&self.0))
    }
}

impl Deref for Hash {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

pub fn blake2b(data: &[u8]) -> Hash {
    let mut hash = [0u8; 32];
    Blake2b::blake2b(&mut hash, data, b"");
    Hash(hash)
}
