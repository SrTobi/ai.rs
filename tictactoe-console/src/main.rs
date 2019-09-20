use tictactoe::*;
use ailib::{Strategy, DeterministicState};
use ailib::strategies::{RandomStrategy, FullSearchStrategy};
use std::collections::HashMap;

fn main() {
    let mut state = TicTacToeState::initial();

    fn to_fn(strat: impl Strategy<TicTacToeState>) 
        -> impl Fn(&TicTacToeState) -> Option<TicTacToeAction> {
        move |state| strat.best_action(state)
    }
    loop {
        println!("{}\n", state);
        
        let mut strats: HashMap<Stone, &dyn Fn(&TicTacToeState) -> Option<TicTacToeAction>> = Default::default();
        let random_strat = to_fn(RandomStrategy::default());
        let fullsearch_strat = to_fn(FullSearchStrategy::default());
        strats.insert(Stone::Circle, &random_strat);
        strats.insert(Stone::Cross, &fullsearch_strat);

        match strats[&state.player()](&state) {
            Some(action) => state.apply_action(&action),
            None => {
                println!("Winner: {}\n", state.winner().unwrap());
                return;
            },
        }
    }
}
