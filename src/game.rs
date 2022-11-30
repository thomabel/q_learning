use crate::q_agent::QLearningAgent;
use crate::q_agent::Value;
use crate::vector::Vector2;
use crate::state::*;
use bracket_lib::random::RandomNumberGenerator;
use ndarray::prelude::*;

pub struct Game {
    player:     Array1<QLearningAgent>,
    board:      State,
    rng:        RandomNumberGenerator,
    epsilon:    Value,
    eta:        Value,
    gamma:      Value,
}

impl Game {
    pub fn new(board_size: Vector2, epsilon: Value, eta: Value, gamma: Value) -> Game {
        // Set up players.
        let mut player_vec = Vec::new();
        for piece in vec![Piece::P1, Piece::P2].iter() {
            let p = QLearningAgent::new(*piece, board_size);
            player_vec.push(p);
        }
        
        let player = Array1::from_vec(player_vec);
        let board = State::new(board_size);
        let rng = RandomNumberGenerator::new();
        Game { player, board, rng, epsilon, eta, gamma }
    }

    /// Plays out a single game, returning the winner.
    pub fn game(&mut self) -> Piece {
        let mut winner = Piece::Empty;
        // run game until there is a winner
        while winner == Piece::Empty {
            winner = self.agent_turn();

            let board = self.board.to_string();
            println!("{}", board);
        }
        winner
    }

    /// Lets the current agent take its turn.
    fn agent_turn(&mut self) -> Piece {
        // Clone current board
        let prev_state = self.board.clone();

        // Find the current player and get them to choose an action.
        let on_play = self.board.on_play();
        let index = on_play.to_index();
        let player = &mut self.player[index];
        let action = player.choose_action(prev_state.clone(), &mut self.rng, self.epsilon).unwrap();
        
        // Update board and clone it using the chosen action.
        self.board.play_mut(&action);
        let state = self.board.clone();
        let winner = state.check_winner(on_play);
        let reward = {
            if winner == on_play {
                1.0
            }
            else if winner == Piece::Draw {
                0.5
            }
            else {
                0.0
            }
        };
            
        // Update the agent's Q-table
        player.update_Q(prev_state, state, &action, reward, self.eta, self.gamma);

        winner
    }

}
