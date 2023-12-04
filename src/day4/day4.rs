use std::collections::HashSet;


fn make_set(values: Vec<usize>) -> HashSet<usize> {
    let mut set = HashSet::new();
    for v in values {
        set.insert(v);
    }
    return set;
}
fn compute_points(s: &String) -> usize {

    let cards = s.split(":").last().unwrap()
        .split("|")
        .map(|s| s.trim().to_string().replace("  ", " "))
        .map(|s| {
            return s.split(" ")
                .map(|cards| cards.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        })
        .collect::<Vec<Vec<usize>>>();

    let winning_nums = make_set(cards.get(0).unwrap().to_vec());

    let mut num_of_winning = 0;
    for v in cards.get(1).unwrap().to_vec() {
        if winning_nums.contains(&v) {
            num_of_winning += 1;
        }
    }
    
    if num_of_winning > 0 {
        return 2_i32.pow(num_of_winning - 1).try_into().unwrap();
    }
    return 0;
}

pub fn execute(data: Vec<String>) {
    let result: usize = data.iter()
        .map(&compute_points)
        .sum();
    dbg!(result);
}
