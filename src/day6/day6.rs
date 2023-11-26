use std::collections::HashSet;

pub fn execute(data: Vec<String>) {
    let str = data.get(0).unwrap();
    let char_array = str.chars().collect::<Vec<char>>();

    let result_part1 = windows_me(4, &char_array);
    dbg!(result_part1);

    let result_part2 = windows_me(14, &char_array);
    dbg!(result_part2);
}

fn windows_me(window_size: usize, char_array: &Vec<char>) -> i32 {
    let mut set: HashSet<&char> = HashSet::with_capacity(window_size);
    let mut result = 0;
    for (i, elements) in char_array.windows(window_size).enumerate() {
        for letter in elements {
            set.insert(letter);
        }
        if set.len() == window_size {
            result = i as i32 + window_size as i32;
            break;
        }
        set.drain();
    }
    return result;
}
