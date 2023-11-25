use std::collections::HashSet;

pub fn execute(data: Vec<String>) {
    let result_part1 = data.clone().iter()
        .fold(0, |acc, e| {
            let (l, r) = e.split_at(e.len()/2);
            let mut i = 0;

            while i < r.len() {
                let r_char = r.as_bytes()[i] as char;

                if l.contains(r_char) {
                    return acc + get_decimal_from_char(r_char);
                }
                i += 1;
            }
            dbg!("no match found, should fix");
            return acc + 1;
        });
    dbg!(result_part1);

    let result_2 = data.chunks(3)
        .fold(0, |acc, e| {
            let set: HashSet<char> = e.get(0).unwrap().chars().collect();
            let set2: HashSet<char> = e.get(1).unwrap().chars().collect();

            let s = e.get(2).unwrap().as_bytes();
            let mut i = 0;

            while i < s.len() {
                let c = s[i] as char;
                if set.contains(&c) && set2.contains(&c) {
                    return acc + get_decimal_from_char(c);
                }
                i += 1;
            }
            return 1;
        });

    dbg!(result_2);
}

fn get_decimal_from_char(c: char) -> u32 {
    let v: u32;
    if c.is_lowercase() {
        v = (c as u32) - 96;
    } else {
        v = (c as u32) - 38;
    }
    return v;
}
