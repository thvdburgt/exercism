pub struct Allergies(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0 & (*allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        let mut score = self.0;
        while score != 0 {
            let n = 1 << score.trailing_zeros();
            if let Ok(allergen) = Allergen::try_from(n) {
                allergies.push(allergen);
            }
            score -= n;
        }

        allergies
    }
}

#[derive(Debug)]
pub struct TryFromU32Error(());

impl TryFrom<u32> for Allergen {
    type Error = TryFromU32Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Allergen::Eggs),
            2 => Ok(Allergen::Peanuts),
            4 => Ok(Allergen::Shellfish),
            8 => Ok(Allergen::Strawberries),
            16 => Ok(Allergen::Tomatoes),
            32 => Ok(Allergen::Chocolate),
            64 => Ok(Allergen::Pollen),
            128 => Ok(Allergen::Cats),
            _ => Err(TryFromU32Error(())),
        }
    }
}
