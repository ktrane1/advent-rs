pub fn execute(data: Vec<String>) {
    let result_part1 = data.iter()
        .map(|line| {
            let mut first: char = char::default();
            let mut second: char = char::default();

            for x in line.chars() {
                match x.to_digit(10) {
                    Some(num) => {
                        first = x;
                        break;
                    },
                    None => {},
                }
            }

            for x in line.chars().rev() {
                match x.to_digit(10) {
                    Some(num) => {
                        second = x;
                        break;
                    },
                    None => {},
                }
            }

            return vec![first, second].into_iter().collect::<String>();
        })
    .map(|s| s.parse::<usize>().unwrap())
        .sum::<usize>();

    dbg!(result_part1);
}
