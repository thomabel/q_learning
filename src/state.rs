use ndarray::prelude::*;
use crate::vector::Vector2;
use crate::piece::Piece;

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

    pub fn reset(&mut self) {
        self.on_play = Piece::P1;
        for piece in self.map.iter_mut() {
            *piece = Piece::Empty;
        }
    }

    /// Checks the action and performs it on the state, changing state in-place.
    pub fn play_mut(&mut self, action: &Action) {
        if self.is_in_bounds(&action.position) && self.is_legal_move(action) {
            self.modify_state(action);
        }
    }

    /// Checks if this state has any empty spaces left.
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

    /// Reports the current state of the game.
    /// Returns the winner.
    pub fn check_winner(&self) -> Piece {
        let mut winner = self.check_lines();
        if self.terminal() && winner == Piece::Empty {
            winner = Piece::Draw;
        }
        winner
    }

    /// Which player is on the play?
    pub fn on_play(&self) -> Piece {
        self.on_play
    }

}

impl State {
    // PRIVATE

    fn is_in_bounds(&self, vector: &Vector2) -> bool {
        let dim = self.map.dim();
        vector.x >= 0 && vector.x < dim.0 as i32 &&
        vector.y >= 0 && vector.y < dim.1 as i32
    }
    
    fn is_legal_move(&self, action: &Action) -> bool {
        let index = action.position.index();
        self.map[[index.0, index.1]] == Piece::Empty
    }

    /// In-place morphing of the state.
    fn modify_state(&mut self, action: &Action) {
        let index = action.position.index();
        self.map[[index.0, index.1]] = action.player;
        self.on_play = match action.player {
            Piece::P1 => Piece::P2,
            Piece::P2 => Piece::P1,
            _ => Piece::Empty,
        };
    }

    /// Checks the state to see if any player has won yet.
    fn check_lines(&self) -> Piece {
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
        let first = map[[0, dim.1 - 1]];
        if first == Piece::Empty {
            return Piece::Empty;
        }
        let mut j = dim.1;
        for i in 0..dim.0 {
            j -= 1;
            if map[[i, j]] != first {
                return Piece::Empty;
            }
        }
        first
    }

}

impl ToString for State {
    fn to_string(&self) -> String {
        let mut string = String::new();
        for row in self.map.rows() {
            for col in row {
                let str = 
                match *col {
                    Piece::P1 => "-X-",
                    Piece::P2 => "-O-",
                    Piece::Empty => "|_|",
                    _ => "???",
                };
                string.push_str(str);
            }
            string.push('\n');    
        }
        string
    }
}
