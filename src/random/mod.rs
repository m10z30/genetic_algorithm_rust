use rand::{distributions::Distribution, Rng};

pub struct AlphanumericAndSpace;

impl Distribution<char> for AlphanumericAndSpace {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 ";
        let idx = rng.gen_range(0..CHARS.len());
        CHARS[idx] as char
    }
}