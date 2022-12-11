#[derive(Debug, Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

pub enum WinState {
    Win,
    Draw,
    Lose,
}

pub enum GameType {
    PlayInResponse,
    WinDrawOrLose,
}

#[derive(Debug, Clone, Copy)]
pub struct Strategy {
    pub opponent: Move,
    pub player: Move,
}

impl From<&str> for Strategy {
    fn from(value: &str) -> Self {
        let parse_element = |s: &str| -> Move {
            let s = s.trim();
            return match s {
                "A" | "X" => Move::Rock,
                "B" | "Y" => Move::Paper,
                "C" | "Z" => Move::Scissors,
                _ => panic!("Input invalid: {}", s),
            };
        };

        let parsed_strategy: Vec<Move> = value.split(" ").map(parse_element).collect();
        return Self {
            opponent: parsed_strategy[0],
            player: parsed_strategy[1],
        };
    }
}

#[derive(Debug)]
pub struct Game {
    pub strategies: Vec<Strategy>,
}

impl Game {
    fn play_in_response_round(strategy: &Strategy) -> i32 {
        let mut score: i32 = 0;

        match (strategy.player, strategy.opponent) {
            (Move::Rock, Move::Paper) => score += 1,
            (Move::Rock, Move::Rock) => score += 1 + 3,
            (Move::Rock, Move::Scissors) => score += 1 + 6,
            (Move::Paper, Move::Scissors) => score += 2,
            (Move::Paper, Move::Paper) => score += 2 + 3,
            (Move::Paper, Move::Rock) => score += 2 + 6,
            (Move::Scissors, Move::Rock) => score += 3,
            (Move::Scissors, Move::Scissors) => score += 3 + 3,
            (Move::Scissors, Move::Paper) => score += 3 + 6,
        };

        return score;
    }

    fn play_win_draw_or_lose_round(strategy: &Strategy) -> i32 {
        let mut score: i32 = 0;
        let move_to_win_state = |mv: Move| -> WinState {
            match mv {
                Move::Rock => WinState::Lose,
                Move::Paper => WinState::Draw,
                Move::Scissors => WinState::Win,
            }
        };
    
        match (move_to_win_state(strategy.player), strategy.opponent) {
            (WinState::Win, Move::Rock) => score += 2 + 6,
            (WinState::Win, Move::Paper) => score += 3 + 6,
            (WinState::Win, Move::Scissors) => score += 1 + 6,
            (WinState::Draw, Move::Rock) => score += 1 + 3,
            (WinState::Draw, Move::Paper) => score += 2 + 3,
            (WinState::Draw, Move::Scissors) => score += 3 + 3,
            (WinState::Lose, Move::Rock) => score += 3,
            (WinState::Lose, Move::Paper) => score += 1,
            (WinState::Lose, Move::Scissors) => score += 2,
        }
        
        return score;
    }

    pub fn play(&self, game_type: GameType) -> i32 {
        match game_type {
            GameType::PlayInResponse => self
                .strategies
                .iter()
                .map(|strategy| Game::play_in_response_round(&strategy))
                .sum(),
            GameType::WinDrawOrLose => self
                .strategies
                .iter()
                .map(|strategy| Game::play_win_draw_or_lose_round(&strategy))
                .sum(),
        }
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let strategies = value.split("\n").map(Strategy::from).collect();
        return Self { strategies };
    }
}
