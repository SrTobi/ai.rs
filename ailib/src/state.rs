use std::fmt::Display;
use crate::Winner;

pub trait State: Clone {
    type Action: Display;
    type Player: Eq + Copy + Display;
    type ActionEffect;
    type ActionEffectIterator: Iterator<Item = Self::ActionEffect>;

    fn possible_actions(&self) -> Vec<Self::Action>;

    fn player(&self) -> Self::Player;
    fn winner(&self) -> Option<Winner<Self::Player>>;

    fn action_effects(&self, action: &Self::Action) -> Self::ActionEffectIterator;
    fn apply_effect(&mut self, effect: &Self::ActionEffect) -> f64;
    fn unapply_effect(&mut self, effect: &Self::ActionEffect);
}