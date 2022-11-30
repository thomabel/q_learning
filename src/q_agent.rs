// 

use std::collections::HashMap;
use bracket_lib::random::RandomNumberGenerator;

use crate::state::*;
use crate::vector::*;
use crate::piece::*;

pub type Value = f64;
type Frequency = u32;
type Legal = bool;
type QTable = HashMap<(State, Action), (Value, Frequency)>;
type Actions = Vec<(Action, Legal)>;

pub struct QLearningAgent {
    q_table:    QTable,
    actions:    Actions,
}

impl QLearningAgent {
    pub fn new(board_size: Vector2, player: Piece) -> QLearningAgent {
        let q_table = HashMap::new();
        let actions = Self::create_actions(board_size, player);
        QLearningAgent { q_table, actions }
    }

    /// Uses the Q-Table to choose the best action, sometimes choosing a random action.
    pub fn choose_action(&mut self, state: State, rng: &mut RandomNumberGenerator, epsilon: Value) -> Action {
        // Set up
        self.update_actions(&state);
        if !Self::legal_actions(&self.actions) {
            panic!("No legal actions");
        }
        let p = rng.range(0., 1.);

        // On-policy/greedy action
        if p > epsilon {
            let index = Self::max_q_action(&self.q_table, &self.actions, state).0;
            self.actions[index].0
        }
        // Random action
        else {
            let len = self.actions.len();
            loop {
                let index = rng.range(0, len);
                let action = self.actions[index];
                if action.1 {
                    return action.0
                }
            }

        }
    }

    /// Updates the Q value using the Bellman Equation.
    pub fn update_q(&mut self, prev_state: State, state: State, action: &Action, 
        reward: Value, eta: Value, gamma: Value) {
        // max_q is the largest q value given the next state and set of available actions.
        let max_q = Self::max_q_action(&self.q_table, &self.actions, state).1;
        let k = (prev_state, *action);

        // Search for the entry in the table.
        match self.q_table.get_mut(&k) {
            // Add new entry.
            None => {
                let mut v = (0., 1);
                v.0 += eta * (reward + gamma * max_q);
                self.q_table.insert(k, v);
            },
            // Update the existing entry.
            Some((q_value, freq)) => {
                *q_value += eta * (reward + gamma * max_q - *q_value);
                *freq += 1;
            },
        };
    }



    /// Creates the initial set of legal actions.
    fn create_actions(board_size: Vector2, player: Piece) -> Vec<(Action, Legal)> {
        let mut actions = Vec::new();
        for i in 0..board_size.x {
            for j in 0..board_size.y {
                let position = Vector2::new(i, j);
                actions.push((Action{player, position}, true));
            }
        }
        actions
    }

    /// Removes actions that are no longer legal.
    fn update_actions(&mut self, state: &State) {
        for (action, legal) in self.actions.iter_mut() {
            *legal = state.get_piece(&action.position) == Piece::Empty;
        }
    }

    /// Chooses the action with the highest Q value given some state.
    /// Returns an index into the action table and the actual Q value;
    fn max_q_action(q_table: &QTable, actions: &Actions, state: State) -> (usize, Value) {
        let mut q_value = 0.;
        let mut index = 0;
        let mut k = (state, actions[0].0);

        for (i, (action, legal)) in actions.iter().enumerate() {
            if *legal {
                // Make sure we choose a legal action.
                if q_value == 0. {
                    index = i;
                }
                // Swap actions in the key, keeping state constant.
                k.1 = *action;
                match q_table.get(&k) {
                    None => (),
                    Some((q, _frq)) => {
                        if *q > q_value {
                            q_value = *q;
                            index = i;
                        }
                    }
                }
            }
        }
        (index, q_value)
    }

    /// Tells if we have any legal actions left.
    fn legal_actions(actions: &Actions) -> bool {
        let mut count = 0;
        for a in actions.iter() {
            if a.1 {
                count += 1;
            }
        }
        count > 0
    }

}
