use std::collections::HashMap;

static NUCLEOTIDES: &'static str = "ACGT";

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(nucleotide) {
        return Err(nucleotide);
    }
    let mut count = 0;
    for n in dna.chars() {
        if !NUCLEOTIDES.contains(n) {
            return Err(n);
        }
        if n == nucleotide {
            count += 1;
        }
    }
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    for n in NUCLEOTIDES.chars() {
        let count = count(n, dna)?;
        counts.insert(n, count);
    }
    Ok(counts)
}
