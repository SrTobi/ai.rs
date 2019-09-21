use std::fmt::Display;
use std::iter::{once, Once};
use crate::{State, Winner};

pub trait DeterministicState: Clone {
    type Action: Display;
    type Player: Eq + Copy + Display;

    fn possible_actions(&self) -> Vec<Self::Action>;

    fn player(&self) -> Self::Player;
    fn winner(&self) -> Option<Winner<Self::Player>>;

    fn apply_action(&mut self, action: &Self::Action);
    fn unapply_action(&mut self, action: &Self::Action);
}

impl <T> State for T
    where
        T: DeterministicState,
        T::Action: Clone
{
    type Action = T::Action;
    type Player = T::Player;
    type ActionEffect = Self::Action;
    type ActionEffectIterator = Once<Self::ActionEffect>;

    fn possible_actions(&self) -> Vec<Self::Action> {
        DeterministicState::possible_actions(self)
    }

    fn player(&self) -> Self::Player {
        DeterministicState::player(self)
    }

    fn winner(&self) -> Option<Winner<Self::Player>> {
        DeterministicState::winner(self)
    }

    fn action_effects(&self, action: &Self::Action) -> Self::ActionEffectIterator {
        // todo replace by reference!
        once(action.clone())
    }

    fn apply_effect(&mut self, effect: &Self::ActionEffect) -> f64 {
        DeterministicState::apply_action(self, effect);
        1.0
    }

    fn unapply_effect(&mut self, effect: &Self::ActionEffect) {
        DeterministicState::unapply_action(self, effect);
    }
}