pub fn parse(input: &str) -> Vec<Vec<i32>> {
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

pub fn get_totals(data: Vec<Vec<i32>>) -> Vec<i32> {
    return data
        .into_iter()
        .map(|group| group.into_iter().reduce(|a, b| a + b).unwrap())
        .collect();
}
