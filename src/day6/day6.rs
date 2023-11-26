use std::collections::HashSet;

pub fn execute(data: Vec<String>) {
    let mut result: i32 = 0;
    let str = data.get(0).unwrap();
    let mut set: HashSet<&char> = HashSet::new();
    for (i, elements) in str.chars().collect::<Vec<char>>().windows(4).enumerate() {
        for letter in elements {
            set.insert(letter); 
        }
        if set.len() == 4 {
            result = i as i32 + 4;
            break
        }
        set.drain();
    }
    dbg!(result);
}
