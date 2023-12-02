fn part1(data: Vec<String>) {
    let result_part1 = data
        .clone()
        .iter()
        .map(|line| {
            let mut first: char = char::default();
            let mut second: char = char::default();

            for x in line.chars() {
                match x.to_digit(10) {
                    Some(_num) => {
                        first = x;
                        break;
                    }
                    None => {}
                }
            }

            for x in line.chars().rev() {
                match x.to_digit(10) {
                    Some(_num) => {
                        second = x;
                        break;
                    }
                    None => {}
                }
            }

            return vec![first, second].into_iter().collect::<String>();
        })
        .map(|s| s.parse::<usize>().unwrap())
        .sum::<usize>();

    dbg!(result_part1);
}

fn digit_to_word_digit(s: &str) -> &str {
    match s {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => s,
    }
}

fn part2(data: Vec<String>) {
    let looking_for = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];

    let result_part2 = data
        .iter()
        .map(|line| {
            let mut first_i_val = "";
            let mut first_i = usize::max_value();

            for l in looking_for.iter() {
                let found = line.find(l);
                match found {
                    Some(x) => {
                        if x < first_i {
                            first_i_val = l;
                            first_i = x;
                        }
                    }
                    None => {}
                }
            }

            let mut last_i_val = "";
            let mut last_i = 0;

            for l in looking_for.iter() {
                let found = line.rfind(l);
                match found {
                    Some(x) => {
                        if x >= last_i {
                            last_i_val = l;
                            last_i = x;
                        }
                    }
                    None => {}
                }
            }

            let r = format!(
                "{}{}",
                digit_to_word_digit(first_i_val),
                digit_to_word_digit(last_i_val)
            );
            return r;
        })
        .map(|s| s.parse::<usize>().unwrap())
        .sum::<usize>();

    dbg!(result_part2);
}

pub fn execute(data: Vec<String>) {
    part1(data.clone());
    part2(data);
}
