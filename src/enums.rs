mod villager;
mod direction;


use rand::{seq::SliceRandom};

pub use villager::Villager;
pub use direction::Direction;

pub struct World {
    pub villager_1: Villager,
    pub villager_2: Villager,
    pub direction_heaven: Direction,
}


impl World {
    pub fn random() -> Self {
        let mut rng = rand::rng(); // or rand::thread_rng() in rand 0.8

        let mut villagers = [Villager::Truth, Villager::False];
        villagers.shuffle(&mut rng);
        let [villager_1, villager_2] = villagers;
        World {
            villager_1,
            villager_2,
            direction_heaven: Direction::random(),
        }
    }
    pub fn check_answer (&self, answer: Direction) {
        if answer == self.direction_heaven {
            // ANSI escape code for green
            print!("\u{001b}[32mCorrect\u{001b}[0m ");
        } else {
            // ANSI escape code for red
            print!("\u{001b}[31mIncorrect\u{001b}[0m ");
        }
        println!(" (Your answer: {}, Actual answer: {})", answer, self.direction_heaven);
    }
}


// As always, we should be able to print it
impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "villager_1: {}", self.villager_1)?;
        writeln!(f, "villager_2: {}", self.villager_2)?;
        writeln!(f, "direction_heaven: {}", self.direction_heaven)
    }
}




