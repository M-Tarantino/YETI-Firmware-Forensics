use rayon::prelude::*;
use crate::util::error::YetiResult;

#[derive(Clone, Default)]
pub struct Signature {
    pub name: String,
    pub magic: Vec<u8>,
    pub category: String,
}

#[derive(Clone, Debug, Default)]
pub struct Candidate {
    pub offset: u64,
    pub name: String,
    pub score: f32,
}

pub struct Scanner { sigs: Vec<Signature> }

impl Scanner {
    pub fn new(sigs: Vec<Signature>) -> Self { Self { sigs } }

    pub fn scan_parallel(&self, mmap: &[u8]) -> YetiResult<Vec<Candidate>> {
        let results = self.sigs.par_iter().flat_map(|sig| {
            let mut matches = Vec::new();
            let magic_len = sig.magic.len();
            if magic_len == 0 { return matches; }

            for i in 0..mmap.len().saturating_sub(magic_len) {
                if mmap[i..i+magic_len] == sig.magic {
                    matches.push(Candidate {
                        offset: i as u64,
                        name: sig.name.clone(),
                        score: 1.0,
                    });
                }
            }
            matches
        }).collect();
        Ok(results)
    }
}