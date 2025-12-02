pub fn entry(input: &str) -> String {
    let mut output: String;
    output = part1(input);
    output += ", ";
    output += &part2(input);
    output
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for pair in input.split(",") {
        let ids: Vec<&str> = pair.split("-").collect();

        for id in ids[0].trim().parse::<u64>().unwrap()..=ids[1].trim().parse::<u64>().unwrap() {
            sum += find_invalid_id(id.to_string());
        }
    }
    sum.to_string()
}

fn find_invalid_id(id: String) -> u64 {
    let count = id.len();
    if count % 2 > 0 {
        0
    } else {
        let (left, right) = id.split_at(count/2);
        if left == right {
            id.parse::<u64>().unwrap()
        } else {
            0
        }
    }
}

fn part2(input: &str) -> String {
    "hell".to_string()
}
