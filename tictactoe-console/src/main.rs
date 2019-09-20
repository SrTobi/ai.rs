use tictactoe::*;
use ailib::{Strategy, DeterministicState};
use ailib::strategies::RandomStrategy;

fn main() {
    let mut state = TicTacToeState::initial();

    loop {
        println!("{}\n", state);
        
        let strat: &dyn Strategy<TicTacToeState, Rating = u32> = &RandomStrategy::new();

        match strat.best_action(&state) {
            Some(action) => state.apply_action(&action),
            None => {
                println!("Winner: {}\n", state.winner().unwrap());
                return;
            },
        }
    }
}
