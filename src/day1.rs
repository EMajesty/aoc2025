pub fn entry(input: &str) -> String {
    let mut output: String;
    output = part1(input);
    output += ", ";
    output += &part2(input);
    output
}

fn line_to_number(line: &str) -> i16 {
    let (char, num) = line.split_at(1);
    let number: i16 = num.parse().expect("not a number");

    if char == "L" {
        -number
    } else {
        number
    }
}

fn part1(input: &str) -> String {
    let mut start = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        start += line_to_number(line);
        start = start.rem_euclid(100);
        if start == 0 { zeroes += 1 }
    }

    zeroes.to_string()
}

fn part2(input: &str) -> String {
    let mut start = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let mut number = line_to_number(line);
        if number > 0 {
            for _ in 0..number {
                start += 1;
                if start == 100 { start = 0 }
                if start == 0 { zeroes += 1 }
            }
        } else {
            number = -number;
            for _ in 0..number {
                start -= 1;
                if start == -1 { start = 99 }
                if start == 0 { zeroes += 1 }
            }
        }
    }

    zeroes.to_string()
}
