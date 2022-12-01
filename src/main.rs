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
mod plotter;

fn main() {
    experiment();
}

fn experiment() {
    use piece::Piece;

    // Initial specifications
    let board_size = vector::Vector2::new(3, 3);
    let eta = 0.1;
    let gamma = 0.9;
    
    // For changing epsilon
    let epsilon = 0.1;
    let delta = 0.01;
    let m = 10;
    
    // Create the board and agents.
    let mut game = game::Game::new(board_size, epsilon, eta, gamma);

    // Define training times.
    let epochs = 100;
    let epoch_games = usize::pow(2, 12);
    let test_games = 10;
    let mut test_data = Vec::<plotter::QResult>::with_capacity(epochs);
 
    // Who is playing and do we print the games.
    let print = false;
    let agent_player = Piece::P2;
    let human_player = Piece::Empty;

    for e in 0..epochs {
        // Training
        let _train_result = epoch(&mut game, epoch_games, print, agent_player, Piece::Empty);

        // Testing
        let test_result = epoch(&mut game, test_games, print, agent_player, human_player);
        test_data.push(test_result);

        // Check if we should decrement epsilon.
        if e % m == 0 && game.epsilon > 0. {
            let temp = game.epsilon - delta;
            if temp <= 0. { game.epsilon = 0.; }
            else { game.epsilon = temp; }
        }
    }

    let file_name = format!("{}_{}", agent_player.to_string(), epochs);
    let title = format!("agent: {}, epochs: {}", agent_player.to_string(), epochs);
    let _error = plotter::visualize(test_data, file_name, title);
    
}

fn epoch(game: &mut game::Game, to_play: usize, print: bool, agent_player: piece::Piece, human_player: piece::Piece) -> plotter::QResult {
    use crate::piece::Piece;
    let mut result = plotter::QResult::new(to_play);

    // Play the games.
    for i in 0..to_play {
        if print { println!("---------- Game {} ----------", i); }
        let winner = game.play(print, agent_player, human_player);
        match winner {
            Piece::P1 => result.p1_win += 1,
            Piece::P2 => result.p2_win += 1,
            Piece::Draw => result.draw += 1,
            _ => ()
        }
        game.reset();
    }

    result
}
