use super::{Game, GameType};

pub fn puzzle_two(data: &str) -> i32 {
    let game = Game::from(data);
    return game.play(GameType::WinDrawOrLose);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_result_based_on_sample_input() {
        let sample_input = include_str!("./data/sample.txt");
        assert_eq!(12, puzzle_two(sample_input));
    }
}
