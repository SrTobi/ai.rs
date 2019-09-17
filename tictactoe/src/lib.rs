#[macro_use]
extern crate enum_display_derive;

use ailib::{Winner, State};
use std::fmt;
use std::fmt::Display;

#[derive(Copy, Clone, Eq, PartialEq, Display, Debug)]
pub enum Stone {
    Circle,
    Cross
}

#[derive(Clone)]
pub struct TicTacToeState {
    fields: [[Option<Stone>; 3]; 3],
    turn: u32,
    winner: Option<Winner<Stone>>,
}

impl TicTacToeState {
    pub fn initial() -> TicTacToeState {
        TicTacToeState {
            fields: Default::default(),
            turn: 0,
            winner: None,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Option<Stone> {
        self.fields[x][y]
    }

    pub fn empty(&self, x: usize, y: usize) -> bool {
        self.at(x, y).is_none()
    }

    pub fn turn(&self) -> u32 {
        self.turn
    }

    pub fn fields(&self) -> &[[Option<Stone>; 3]; 3] {
        &self.fields
    }
}

pub struct TicTacToeAction {
    x: usize,
    y: usize,
}

impl TicTacToeAction {
    fn new(x: usize, y: usize) -> TicTacToeAction {
        TicTacToeAction { x, y }
    }
}

impl State for TicTacToeState {
    type Action = TicTacToeAction;
    type Player = Stone;

    fn winner(&self) -> Option<Winner<Stone>> {
        self.winner
    }

    fn player(&self) -> Stone {
        match self.turn() % 2 {
            0 => Stone::Circle,
            1 => Stone::Cross,
            _ => unreachable!(),
        }
    }

    fn possible_actions(&self) -> Vec<Self::Action> {
        if self.winner.is_some() {
            return Vec::new();
        }
        if self.turn == 0 {
            vec!(TicTacToeAction::new(0, 0), TicTacToeAction::new(1, 0), TicTacToeAction::new(1, 1))
        } else {
            const POSITIONS: [(usize, usize); 9] = [
                (0, 0), (1, 0), (2, 0),
                (0, 1), (1, 1), (2, 1),
                (0, 2), (1, 2), (2, 2),
            ];

            POSITIONS
                .iter()
                .filter(|(x, y)| self.empty(*x, *y))
                .map(|(x, y)| TicTacToeAction::new(*x, *y))
                .collect()
        }
    }

    fn act(&mut self, TicTacToeAction {x, y} : Self::Action) {
        assert!(self.empty(x, y));
        assert!(self.winner().is_none());
        let player = self.player();
        self.fields[x][y] = Some(self.player());
        self.turn += 1;

        // check if this was a winning move
        const DIRS: [(i16, i16); 4] = [
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for (rx, ry) in &DIRS {
            let mut inrow = 1;
            for mul in &[1, -1] {
                let (mut cx, mut cy) = (x as i16, y as i16);
                for _ in 0..2 {
                    cx += rx * mul;
                    cy += ry * mul;
                    if cx < 0 || 3 <= cx || cy < 0 || 3 <= cy
                        || self.at(cx as usize, cy as usize) != Some(player) {
                        break;
                    }
                    inrow += 1;
                }
            }
            if inrow >= 3 {
                self.winner = Some(Winner::Player(player));
                return;
            }
        }

        if self.turn == 9 {
            self.winner = Some(Winner::Draw)
        }
    }
}

impl fmt::Display for TicTacToeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..5 {
            for y in 0..5 {
                f.write_str(match (x % 2 == 0, y % 2 == 0) {
                    (false, false) => "+",
                    (true, false) => "|",
                    (false, true) => "-",
                    (true, true) =>
                        match self.at(x / 2, y / 2) {
                            Some(Stone::Circle) => "O",
                            Some(Stone::Cross) => "X",
                            None => " "
                        }
                })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}