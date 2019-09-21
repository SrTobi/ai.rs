
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