use crate::*;
use rand::Rng;

pub struct RandomStrategy;

impl RandomStrategy {
    pub fn new() -> RandomStrategy {
        RandomStrategy
    }
}

impl Default for RandomStrategy {
    fn default() -> RandomStrategy {
        RandomStrategy::new()
    }
}

impl <S: State> Strategy<S> for RandomStrategy {
    type Rating = u32;

    fn rated_actions(&self, state: &S) -> Vec<(S::Action, Self::Rating)> {
        let mut rng = rand::thread_rng();
        state
            .possible_actions()
            .into_iter()
            .map(|action| (action, rng.gen::<u32>()))
            .collect()
    }
}
