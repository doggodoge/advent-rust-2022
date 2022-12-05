use super::utils::{parse, get_totals};

pub fn puzzle_one(input: &str) -> i32 {
    let totals: Vec<i32> = get_totals(parse(input));

    return totals.into_iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("./data/sample.txt");

    #[test]
    fn should_equal_24000_with_sample_input() {
        assert_eq!(24000, puzzle_one(SAMPLE_INPUT));
    }
}
