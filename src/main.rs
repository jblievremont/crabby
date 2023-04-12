fn main() {
    // define your games a and b
    let a = Game::Rock;
    let b = Game::Scissor;
    // call play function with arguments
    let result = play(&a, &b);
    // display result
    println!("Result of playing {:?} vs {:?} is {:?}", a, b, result);
}

#[derive(Debug)]
enum Game {
    Rock,
    Paper,
    Scissor
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Win,
    Lost,
    Draw
}

fn play(a: &Game, b: &Game) -> GameResult {
    match(a, b) {
        (Game::Rock, Game::Rock) | (Game::Paper, Game::Paper) | (Game::Scissor, Game::Scissor) => GameResult::Draw,
        (Game::Rock, Game::Scissor) | (Game::Paper, Game::Rock) | (Game::Scissor, Game::Paper) => GameResult::Win,
        (Game::Rock, Game::Paper) | (Game::Paper, Game::Scissor) | (Game::Scissor, Game::Rock) => GameResult::Lost
    }
} 

#[cfg(test)]
mod tests {
    use super::play;
    use super::Game;
    use super::GameResult;

    macro_rules! play_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (a, b, expected) = $value;
                assert_eq!(expected, play(a, b));
            }
        )*
        }
    }
    
    play_tests! {
        play_r_r_d: (Game::Rock, Game::Rock, GameResult::Draw),
        play_p_p_d: (Game::Paper, Game::Paper, GameResult::Draw),
        play_s_s_d: (Game::Scissor, Game::Scissor, GameResult::Draw),
        play_r_s_w: (Game::Rock, Game::Scissor, GameResult::Win),
        play_p_r_w: (Game::Paper, Game::Rock, GameResult::Win),
        play_s_p_w: (Game::Scissor, Game::Paper, GameResult::Win),
        play_r_p_l: (Game::Rock, Game::Paper, GameResult::Lost),
        play_p_s_l: (Game::Paper, Game::Scissor, GameResult::Lost),
        play_s_r_l: (Game::Scissor, Game::Rock, GameResult::Lost),
    }
}
