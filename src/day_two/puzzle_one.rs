use super::{Game, GameType};

pub fn puzzle_one(data: &str) -> i32 {
    let game = Game::from(data);
    return game.play(GameType::PlayInResponse);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_correct_result_based_on_sample_input() {
        let sample_input = include_str!("./data/sample.txt");
        assert_eq!(15, puzzle_one(sample_input));
    }
}
