use crate::q_agent::QLearningAgent;
use crate::vector::Vector2;
use crate::state::*;
use ndarray::prelude::*;

pub struct Game {
    player: Array1<QLearningAgent>,
    board: State,
}

impl Game {
    pub fn new(board_size: Vector2) -> Game {
        // Set up players.
        let mut player_vec = Vec::new();
        for piece in vec![Piece::P1, Piece::P2].iter() {
            let p = QLearningAgent::new(*piece, board_size);
            player_vec.push(p);
        }
        
        let player = Array1::from_vec(player_vec);
        let board = State::new(board_size);
        Game { player, board }
    }

    pub fn play_turn(&mut self) {

    }

    fn agent_turn(&mut self) {
        
    }
}
