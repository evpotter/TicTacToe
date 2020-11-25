use std::fmt;

pub enum Player {
    P1,
    P2,
}

impl Player {
    pub fn get_mark(&self) -> char {
        match self {
            Player::P1 => 'x',
            Player::P2 => 'o',
        }
    }

    pub fn get_next(&self) -> Player {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Player::P1 => 1,
            Player::P2 => 2,
        };
        write!(f, "{}", output)
    }
}
