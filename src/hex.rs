use std::fmt::{Formatter, LowerHex, Result};

// Simple wrapper around slices to allow printing as hex strings.
pub struct Hex<'a>(pub &'a [u8]);

impl<'a> LowerHex for Hex<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;
    use super::*;

    #[test]
    fn it_works() {
        let original = b"\x01\x23\x45\x67\x89\xab\xcd\xef";
        let mut s = String::new();
        write!(&mut s, "{:x}", Hex(original)).expect("Problem writing");
        assert_eq!(s, "0123456789abcdef".to_string());
    }
}
