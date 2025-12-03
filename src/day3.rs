pub fn entry(input: &str) -> String {
    let mut output: String;
    output = part1(input);
    output += ", ";
    output += &part2(input);
    output
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    let mut first: u64 = 0;
    let mut second: u64 = 0;
    for line in input.lines() {
        println!("{:?}", line);
        for d in line.chars() {
            first = second;
            let num: u64 = d.to_digit(10).unwrap().into();
            if num > second { second = num; }
        }
        println!("{}{}", first,second);
        sum += format!("{first}{second}").parse::<u64>().unwrap();
    }
    sum.to_string()
}

fn part2(input: &str) -> String {
    "hell".to_string()
}
