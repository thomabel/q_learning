
use ndarray::prelude::*;
use crate::vector::Vector2;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Piece {
    Empty, P1, P2
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Action {
    pub player: Piece,
    pub position: Vector2,
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct State {
    map: Array2<Piece>,
    on_play: Piece,
}

impl State {

    // PUBLIC
    pub fn new(board_size: Vector2) -> State {
        let map = Array2::from_elem(board_size.dim(), Piece::Empty);
        let on_play = Piece::P1;
        State { map, on_play }
    }

    /// Performs a legal action on this state and returns as a new state, or error if the action was malformed.
    pub fn play(&self, action: Action) -> Result<State, String> {
        if self.is_in_bounds(&action.position) && self.is_legal_move(&action) {
            Ok(self.modify_state(&action))
        }
        else {
            Err("Not a legal move.".to_string())
        }
        
    }

    /// Checks if this state is a terminal state.
    pub fn terminal(&self) -> bool {
        for spot in self.map.iter() {
            if *spot == Piece::Empty {
                return false;
            }
        }
        true
    }

    /// Returns a piece given a position vector.
    pub fn get_piece(&self, position: &Vector2) -> Piece {
        let index = position.index();
        self.map[[index.0, index.1]]
    }

    /// Checks the state to see if any player has won yet.
    pub fn check_winner(&self) -> Piece {
        // Game must be over.
        if !self.terminal() {
            return Piece::Empty;
        }
        // check rows
        for line in self.map.rows() {
            let result = Self::check_line(line);
            if result != Piece::Empty {
                return result;
            }
        }
        // check columns
        for line in self.map.columns() {
            let result = Self::check_line(line);
            if result != Piece::Empty {
                return result;
            }
        }
        // normal diagonal
        let result = Self::check_line(self.map.diag());
        if result != Piece::Empty {
            return result;
        }

        // weird diagonal
        let result = Self::check_diag(&self.map);
        if result != Piece::Empty {
            return result;
        }

        Piece::Empty
    }

    

    // PRIVATE

    /// Determines if the vector is in bounds of the play area.
    /// Play area is of size mxn with indicies between 0..m and 0..n
    fn is_in_bounds(&self, vector: &Vector2) -> bool {
        let dim = self.map.dim();
        vector.x >= 0 && vector.x < dim.0 as i32 &&
        vector.y >= 0 && vector.y < dim.1 as i32
    }
    
    /// When playing only legal moves can be performed.
    fn is_legal_move(&self, action: &Action) -> bool {
        let index = action.position.index();
        self.map[[index.0, index.1]] == Piece::Empty
    }

    /// Creates a new state by arbitrarily applying some action.
    fn modify_state(&self, action: &Action) -> State {
        let mut map = self.map.clone();
        let index = action.position.index();
        map[[index.0, index.1]] = action.player;
        let on_play = match action.player {
            Piece::P1 => Piece::P2,
            Piece::P2 => Piece::P1,
            Piece::Empty => Piece::Empty,
        };
        State { map, on_play }
    }

    fn check_line(line: ArrayView1<Piece>) -> Piece {
        let first = line[0];
        if first == Piece::Empty {
            return Piece::Empty;
        }
        for piece in line {
            if *piece != first {
                return Piece::Empty;
            }
        }
        first
    }

    fn check_diag(map: &Array2<Piece>) -> Piece {
        let dim = map.dim();
        let first = map[[0, dim.1]];
        if first == Piece::Empty {
            return Piece::Empty;
        }
        let mut j = dim.1;
        for i in 0..dim.0 {
            if map[[i, j]] != first {
                return Piece::Empty;
            }
            j -= 1;
        }
        first
    }

}
