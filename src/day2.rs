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

        for id in ids[0].trim().parse::<u128>().unwrap()..=ids[1].trim().parse::<u128>().unwrap() {
            sum += find_invalid_id(id.to_string());
        }
    }
    sum.to_string()
}

fn find_invalid_id(id: String) -> u128 {
    let count = id.len();
    if count % 2 > 0 {
        0
    } else {
        let (left, right) = id.split_at(count/2);
        if left == right {
            id.parse::<u128>().unwrap()
        } else {
            0
        }
    }
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    for pair in input.split(",") {
        let ids: Vec<&str> = pair.split("-").collect();

        for id in ids[0].trim().parse::<u128>().unwrap()..=ids[1].trim().parse::<u128>().unwrap() {
            sum += find_invalid_id2(id.to_string());
        }
    }
    sum.to_string()
}

fn find_invalid_id2(id: String) -> u128 {
    let count = id.len();
    if count % 2 > 0 {
        0
    } else {
        for i in 2..=count {
            let parts = split_into_n(&id, i);
            if parts.windows(2).all(|w| w[0] == w[1]) {
                return id.parse::<u128>().unwrap();
            }
        }
        0
    }
}

fn split_into_n(id: &str, n: usize) -> Vec<&str> {
    let count = id.len();
    let mut parts = Vec::with_capacity(n);
    let mut start = 0;

    for i in 1..n {
        let idx = i * count / n;
        parts.push(&id[start..idx]);
        start = idx;
    }
    parts.push(&id[start..count]);
    parts
}
