use std::collections::HashMap;

static NUCLEOTIDES: &'static str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(nucleotide) {
        return Err(nucleotide);
    }

    dna.chars().try_fold(0, |acc, n| {
        if !NUCLEOTIDES.contains(n) {
            return Err(n);
        }
        Ok(if n == nucleotide { acc + 1 } else { acc })
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    NUCLEOTIDES.chars().try_fold(HashMap::new(), |mut acc, n| {
        acc.insert(n, count(n, dna)?);
        Ok(acc)
    })
}
