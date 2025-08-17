pub struct Allergies {
    score: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (*allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies = Vec::new();
        let mut score = self.score;
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
