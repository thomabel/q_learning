/*
Thomas Abel
Artificial Intelligence
2022-11-23
*/

mod q_agent;
mod state;
mod vector;
mod game;
mod piece;

fn main() {
    experiment();
}

fn experiment() {
    use piece::Piece;

    // Initial specifications
    let board_size = vector::Vector2::new(3, 3);
    let epsilon = 0.1;
    let eta = 0.1;
    let gamma = 0.9;
    let print = true;
    let agent = Piece::P2;
    let human = Piece::Empty;
    
    // The players.
    let mut game = game::Game::new(board_size, epsilon, eta, gamma);

    // Tracking stats.
    let games_to_play = usize::pow(2, 12);
    let mut p1_win = 0;
    let mut p2_win = 0;
    let mut draw = 0;

    // Play the games.
    for i in 0..games_to_play {
        println!("---------- Game {} ----------", i);
        let winner = game.play(print, agent, human);
        match winner {
            Piece::P1 => p1_win += 1,
            Piece::P2 => p2_win += 1,
            Piece::Draw => draw += 1,
            _ => ()
        }
        game.reset();
    }

    // Analyze the results.
    let denom = games_to_play as f32;
    let p1_ratio = p1_win as f32 / denom * 1000.;
    let p2_ratio = p2_win as f32 / denom * 1000.;
    let draw_ratio = draw as f32 / denom * 1000.;

    println!("Total / P1W / P2W / Draw :: {:6} / {} ({:3.3}) / {} ({:3.3}) / {} ({:3.3})", 
        games_to_play, 
        p1_win, p1_ratio,
        p2_win, p2_ratio,
        draw, draw_ratio
    );
}
