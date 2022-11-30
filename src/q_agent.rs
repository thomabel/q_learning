// 

use std::collections::HashMap;
use bracket_lib::random::RandomNumberGenerator;

use crate::state::*;
use crate::vector::*;

type Value = f64;
type Frequency = u32;
type Legal = bool;
type QTable = HashMap<(State, Action), (Value, Frequency)>;
type Actions = Vec<(Action, Legal)>;

pub struct QLearningAgent {
    Q:          QTable,
    player:     Piece,
    board_size: Vector2,
    actions:    Actions,
}

impl QLearningAgent {
    pub fn new(player: Piece, board_size: Vector2) -> QLearningAgent {
        let Q = HashMap::new();
        let actions = Vec::new();
        QLearningAgent { Q, player, board_size, actions }
    }

    /// Uses the Q-Table to choose the best action, sometimes choosing a random action.
    pub fn choose_action(&self, state: State, 
        rng: &mut RandomNumberGenerator, epsilon: f32) -> Result<Action, String> {

        let p = rng.range(0., 1.);
        if p > epsilon {
            let index = Self::max_Q_action(&self.Q, &self.actions, state).0;
            Ok(self.actions[index].0)
        }
        else {
            let len = self.actions.len();
            loop {
                let index = rng.range(0, len);
                let action = self.actions[index];
                if action.1 {
                    return Ok(action.0)
                }
            }

        }
    }

    /// Updates the Q value using the Bellman Equation.
    pub fn update_Q(&mut self, prev_state: State, state: State, action: &Action, 
        reward: Value, eta: Value, gamma: Value) {
        // max_q is the largest q value given the next state and set of available actions.
        let max_q = Self::max_Q_action(&self.Q, &self.actions, state).1;
        let k = (prev_state, *action);

        // Search for the entry in the table.
        match self.Q.get_mut(&k) {
            // Add new entry.
            None => {
                let mut v = (0., 1);
                v.0 += eta * (reward + gamma * max_q);
                self.Q.insert(k, v);
            },
            // Update the existing entry.
            Some((q_value, freq)) => {
                *q_value += eta * (reward + gamma * max_q - *q_value);
                *freq += 1;
            },
        };
    }



    /// Creates the initial set of legal actions.
    fn create_actions(&mut self) -> Vec<(Action, Legal)> {
        let mut actions = Vec::new();
        for i in 0..self.board_size.x {
            for j in 0..self.board_size.y {
                let player = self.player.clone();
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
    fn max_Q_action(Q: &QTable, actions: &Actions, state: State) -> (usize, Value) {
        let mut q_value = 0.;
        let mut index = 0;
        let mut k = (state, actions[0].0);

        for (i, (action, legal)) in actions.iter().enumerate() {
            if *legal {
                k.1 = *action;
                match Q.get(&k) {
                    None => (),
                    Some((q, frq)) => {
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

    

}
