use core::cmp::min;
use std::cmp::max;
use crate::*;
use noisy_float::prelude::*;

#[derive(Builder, Copy, Clone, PartialEq)]
pub struct FullSearchStrategy {
    #[builder(default = "n64(1.0)")]
    win_reward: N64,

    #[builder(default = "n64(0.2)")]
    draw_reward: N64,

    #[builder(default = "n64(0.0)")]
    loose_reward: N64,
}

impl FullSearchStrategy {
    pub fn new() -> FullSearchStrategy {
        Default::default()
    }

    fn rate_action<S: State>(&self, a: N64, b: N64, state: &mut S, action: &S::Action, player: S::Player) -> N64 {
        state
            .action_effects(action)
            .map(|effect| {
                let possibility = n64(state.apply_effect(&effect));
                let rating = self.rate_state(a, b, state, player);
                state.unapply_effect(&effect);
                possibility * rating
            })
            .sum()
    }

    fn rate_state<S: State>(&self, mut a: N64, mut b: N64, state: &mut S, player: S::Player) -> N64 {
        if let Some(result) = state.winner() {
            match result {
                Winner::Draw => return self.draw_reward,
                Winner::Player(winner) => {
                    return
                        if winner == player {
                            self.win_reward
                        } else {
                            self.loose_reward
                        }
                }
            }
        }

        let try_win = state.player() == player;

        let actions = state.possible_actions();
        let mut best = 
            if try_win {
                -N64::infinity()
            } else {
                N64::infinity()
            };
        assert!(!actions.is_empty());
        for action in actions {
            let rate = self.rate_action(a, b, state, &action, player);
            if try_win {
                a = max(a, rate);
                best = max(rate, best);
            } else {
                b = min(b, rate);
                best = min(rate, best)
            };
            if a >= b {
                break
            }
        }
        best
    }
}


impl <S: State> Strategy<S> for FullSearchStrategy {
    type Rating = N64;

    fn rated_actions(&self, state: &S) -> Vec<(S::Action, Self::Rating)> {
        let mut mut_state = state.clone();
        state
            .possible_actions()
            .into_iter()
            .map(|action| {
                let rating = self.rate_action(-N64::infinity(), N64::infinity(), &mut mut_state, &action, state.player());
                (action, rating)
            })
            .collect()
    }
}

impl Default for FullSearchStrategy {
    fn default() -> FullSearchStrategy {
        FullSearchStrategyBuilder::default()
            .build()
            .unwrap()
    }
}