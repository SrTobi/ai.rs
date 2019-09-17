use rand::Rng;
use std::fmt;



#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Winner<P: fmt::Display> {
    Player(P),
    Draw,
}

impl <P: fmt::Display> fmt::Display for Winner<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Winner::Player(player) => player.fmt(f)?,
            Winner::Draw => f.write_str("Draw")?,
        }
        Ok(())
    }
}

pub trait State: Clone {
    type Action;
    type Player: fmt::Display;

    fn possible_actions(&self) -> Vec<Self::Action>;
    fn act(&mut self, action: Self::Action);
    fn player(&self) -> Self::Player;
    fn winner(&self) -> Option<Winner<Self::Player>>;
}

pub trait Rated {
    type Rating: Ord + Copy;
    fn rating(&self) -> Self::Rating;
}

pub trait Strategy<State: crate::State> {
    type Rating: Ord + Copy;

    fn rated_actions(&self, state: &State) -> Vec<(State::Action, Self::Rating)>;

    fn best_action(&self, state: &State) -> Option<State::Action> {
        self.rated_actions(state)
            .into_iter()
            .max_by_key(|(_, r)| *r)
            .map(|(action, _)| action)
    }
}



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


/*

pub struct FullSearchStrategy;

impl FullSearchStrategy {
    pub fn new() -> FullSearchStrategy {
        FullSearchStrategy
    }
}

impl <S: State> Strategy<S> for FullSearchStrategy {
    type Rating = bool;

    fn rated_actions(&self, state: &S) -> Vec<(S::Action, Self::Rating)> {
        let actions = state.possible_actions();
        for action in actions {

        }

        unimplemented!();
    }

    fn find_best_action(&self, state: &S)
}

*/



mod test {
    #![allow(dead_code)]

    struct StateHandle<'s, State> {
        state: &'s mut State,
    }

    impl <'s, S> Drop for StateHandle<'s, S> {
        fn drop(&mut self) {
        }
    }

    struct Action;

    impl Action {
        fn act<'prev, 'pp>(&self, h: &'prev mut StateHandle<'pp, u32>) -> StateHandle<'prev, u32> {
            *h.state += 1;
            StateHandle { state: &mut *h.state }
        }
    }

    fn test() {
        let mut state = 0;
        let action = Action;

        let mut h0 = StateHandle { state: &mut state };
        let mut h1 = action.act(&mut h0);
        let h2 = action.act(&mut h1);
        drop(h2);
        *h1.state += 1
    }
}