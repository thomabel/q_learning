#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Piece {
    Draw = -1, 
    Empty = 0, 
    P1, P2
}
impl Piece {
    pub fn to_index(self) -> usize {
        match self {
            Piece::P1 => 0,
            Piece::P2 => 1,
            _ => 0,
        }
    }
}
impl ToString for Piece {
    fn to_string(&self) -> String {
        match *self {
            Piece::Draw => "Draw",
            Piece::Empty => "Empty",
            Piece::P1 => "Player 1",
            Piece::P2 => "Player 2",
        }.to_string()
    }
}