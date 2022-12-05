use super::utils::{parse, get_totals};

pub fn puzzle_two(input: &str) -> i32 {
    let mut totals: Vec<i32> = get_totals(parse(input));

    totals.sort_unstable();
    return totals.iter().rev().take(3).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("./data/sample.txt");

    #[test]
    fn should_return_valid_result_using_sample_input() {
        assert_eq!(45000, puzzle_two(SAMPLE_INPUT));
    }
}
