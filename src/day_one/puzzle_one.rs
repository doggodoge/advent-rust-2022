fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut groups = Vec::new();
    let mut group = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            groups.push(group);
            group = Vec::new();
        } else {
            let number = line.parse::<i32>().unwrap();
            group.push(number);
        }
    }
    groups.push(group);

    return groups;
}

pub fn puzzle_one(input: &str) -> i32 {
    let totals: Vec<i32> = parse(input)
        .into_iter()
        .map(|group| group.into_iter().reduce(|a, b| a + b).unwrap())
        .collect();

    return totals.iter().max().unwrap().clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_equal_24000_with_sample_input() {
        let sample_input = include_str!("./data/sample.txt");
        assert_eq!(24000, puzzle_one(sample_input));
    }
}
