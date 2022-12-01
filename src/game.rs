use crate::q_agent::QLearningAgent;
use crate::q_agent::Value;
use crate::piece::Piece;
use crate::vector::Vector2;
use crate::state::*;
use bracket_lib::random::RandomNumberGenerator;
use ndarray::prelude::*;

pub struct Game {
    player:     Array1<QLearningAgent>,
    board:      State,
    rng:        RandomNumberGenerator,
    eta:        Value,
    gamma:      Value,
    pub epsilon:    Value,
}

impl Game {
    pub fn new(board_size: Vector2, epsilon: Value, eta: Value, gamma: Value) -> Game {
        // Set up players.
        let mut player_vec = Vec::new();
        for piece in vec![Piece::P1, Piece::P2].iter() {
            let p = QLearningAgent::new(board_size, *piece);
            player_vec.push(p);
        }
        
        let player = Array1::from_vec(player_vec);
        let board = State::new(board_size);
        let rng = RandomNumberGenerator::new();
        Game { player, board, rng, epsilon, eta, gamma }
    }

    /// Resets the board in-place.
    pub fn reset(&mut self) {
        self.board.reset();
    }

    /// Plays out a single game, returning the winner.
    pub fn play(&mut self, print: bool, agent_player: Piece, _human_player: Piece) -> Piece {
        // Stores all of the states and vectors of a game.
        let mut state_history = Vec::<State>::with_capacity(10);
        let mut action_history = Vec::<Action>::with_capacity(9);
        
        state_history.push(self.board.clone());
        let mut winner = Piece::Empty;

        while winner == Piece::Empty {
            // agent_turn mutates the board state in-place
            let on_play = self.board.on_play();
            let temp = {
                if agent_player == on_play {
                    self.agent_turn()
                }
                else {
                    // choose a random move
                    let action = self.player[on_play.to_index()].choose_random_action(&mut self.rng);
                    let winner = Self::update_board(&mut self.board, &action).0;
                    (winner, action)
                }
            };
            state_history.push(self.board.clone());
            action_history.push(temp.1);
            winner = temp.0;
        }

        // Print
        if print {
            for state in state_history.iter() {
                println!("{}", state.to_string());
            }
        }

        let reward = if winner == Piece::Draw {
            0.5
        }
        else {
            -0.5
        };
        self.backprop(state_history, action_history, reward);

        winner
    }


    /// Lets the current agent take its turn.
    /// Returns the winning piece and action taken.
    fn agent_turn(&mut self) -> (Piece, Action) {
        // Clone current board
        let prev_state = self.board.clone();

        // Find the current player and get them to choose an action.
        let player = &mut self.player[self.board.on_play().to_index()];
        let action = player.choose_action(prev_state.clone(), &mut self.rng, self.epsilon);
        let (winner, reward) = Self::update_board(&mut self.board, &action);
        let state = self.board.clone();

        // Update the agent's Q-table
        player.update_q(prev_state, state, &action, reward, self.eta, self.gamma);

        (winner, action)
    }

    /// Uses the final played turn of the losing player to update its Q-table.
    fn backprop(&mut self, state_history: Vec<State>, action_history: Vec<Action>, reward: Value) {
        // Get the correct 2 states.
        let mut state_iter = state_history.into_iter().rev();
        state_iter.next();
        let state = state_iter.next().unwrap();
        let prev_state = state_iter.next().unwrap();

        // Get the correct action that was between them.
        let mut action_iter = action_history.iter().rev();
        let action = action_iter.nth(2).unwrap();
        let player = &mut self.player[action.player.to_index()];
        player.update_q(prev_state, state, action, reward, self.eta, self.gamma)
    }

    /// Updates the board state in-place.
    /// Returns a winner and the reward value for that action.
    fn update_board(board: &mut State, action: &Action) -> (Piece, Value) {
        // Update board and clone it using the chosen action.
        board.play_mut(action);
        let winner = board.check_winner();
        let reward = {
            if winner == action.player {
                1.0
            }
            else if winner == Piece::Draw {
                0.5
            }
            else {
                0.0
            }
        };
        (winner, reward)
    }

}
