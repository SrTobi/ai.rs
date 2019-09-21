use std::fmt::{Formatter, Result, Display};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Winner<P: Display> {
    Player(P),
    Draw,
}

impl <P: Display> Display for Winner<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Winner::Player(player) => player.fmt(f)?,
            Winner::Draw => f.write_str("Draw")?,
        }
        Ok(())
    }
}