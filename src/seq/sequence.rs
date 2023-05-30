pub use crate::seq::dna;
use std::fmt;

use bio::data_structures::bitenc::BitEnc;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Seq(BitEnc);

impl Seq {
    pub fn new() -> Self {
        // Self(BitEnc::new(2))

        // Use a default size of 100 to represent a typical seq length
        Self(BitEnc::with_capacity(2, 80))
    }

    pub fn with_capacity(n: usize) -> Self {
        Self(BitEnc::with_capacity(2, n))
    }

    pub fn push(&mut self, b: u8) {
        self.0.push(dna::a_to_b(b))
    }

    pub fn add_iter(&mut self, i: impl Iterator<Item = u8>) {
        i.map(dna::a_to_b).for_each(|x| self.0.push(x));
    }

    pub fn from_string(s: String) -> Self {
        let length = s.len();
        let mut seq = Self::with_capacity(length);
        seq.add_iter(s.bytes());
        seq
    }
}

impl fmt::Display for Seq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self
            .0
            .iter()
            .map(dna::b_to_a)
            .collect::<Vec<&str>>()
            .join("");
        write!(f, "{}", s)
    }
}
