use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cmd = args.get(1).expect("Please provide a command in (greet, chifoumi)");

    match cmd.as_str() {
        "greet" => {
            let name = args.get(2).expect("Name is required for greetings");
            println!("{}", greets(&name));
        }
        "chifoumi" => {
            let p_one = args.get(2).expect("Player one is required for chifoumi");
            let p_two = args.get(3).expect("Player two is required for chifoumi");
            // cast to Game enum
            let p_one: Game = Game::try_from(p_one).unwrap();
            let p_two: Game = Game::try_from(p_two).unwrap();

            let result = play(&p_one, &p_two);
            println!("p1 vs p2 : {:?}", result);
        }
        _ => eprintln!("Not supported command"),
    }
}

impl TryFrom<&String> for Game {
    type Error = String;

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "rock" => Ok(Game::Rock),
            "paper" => Ok(Game::Paper),
            "scissors" => Ok(Game::Scissors),
            _ => Err("Invalid game".to_string()),
        }
    }
}

fn greets(who: &str) -> String {
    format!("Hello, {who}!")
}

#[derive(Debug)]
enum Game {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, PartialEq)]
enum GameResult {
    Win,
    Lost,
    Draw
}

fn play(a: &Game, b: &Game) -> GameResult {
    match(a, b) {
        (Game::Rock, Game::Rock) | (Game::Paper, Game::Paper) | (Game::Scissors, Game::Scissors) => GameResult::Draw,
        (Game::Rock, Game::Scissors) | (Game::Paper, Game::Rock) | (Game::Scissors, Game::Paper) => GameResult::Win,
        (Game::Rock, Game::Paper) | (Game::Paper, Game::Scissors) | (Game::Scissors, Game::Rock) => GameResult::Lost
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
                assert_eq!(expected, play(&a, &b));
            }
        )*
        }
    }
    
    play_tests! {
        play_r_r_d: (Game::Rock, Game::Rock, GameResult::Draw),
        play_p_p_d: (Game::Paper, Game::Paper, GameResult::Draw),
        play_s_s_d: (Game::Scissors, Game::Scissors, GameResult::Draw),
        play_r_s_w: (Game::Rock, Game::Scissors, GameResult::Win),
        play_p_r_w: (Game::Paper, Game::Rock, GameResult::Win),
        play_s_p_w: (Game::Scissors, Game::Paper, GameResult::Win),
        play_r_p_l: (Game::Rock, Game::Paper, GameResult::Lost),
        play_p_s_l: (Game::Paper, Game::Scissors, GameResult::Lost),
        play_s_r_l: (Game::Scissors, Game::Rock, GameResult::Lost),
    }
}
