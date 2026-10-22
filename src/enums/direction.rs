use rand::random_bool;

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn random() -> Self {
        if random_bool(0.5) {
            Direction::Left
        } else {
            Direction::Right
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}

