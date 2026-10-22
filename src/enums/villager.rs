#[derive(PartialEq, Copy, Clone)]
pub enum Villager {
    Truth,
    False,
}

impl Villager {
    pub fn eval(&self, val: bool) -> bool {
        match self {
            Villager::Truth => val,
            Villager::False => !val,
        }
    }
    pub fn ask_equal<T: PartialEq>(&self, v1: T, v2: T) -> bool {
        match self {
            Villager::Truth => v1 == v2,
            Villager::False => v1 != v2,
        }
    }
}


// For printing it
impl std::fmt::Display for Villager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Villager::Truth => write!(f, "Truth"),
            Villager::False => write!(f, "False"),
        }
    }
}
