use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    instances: usize,
    held: Vec<usize>,
    winning_nums: HashSet<usize>,
}

impl Card {
    fn inc(&mut self, i: usize) {
        self.instances += i;
    }
}

fn make_set(values: Vec<usize>) -> HashSet<usize> {
    let mut set = HashSet::new();
    for v in values {
        set.insert(v);
    }
    return set;
}

fn part1_points_calculator(held_card: Vec<usize>, winning_nums: HashSet<usize>) -> usize {
    let mut num_of_winning = 0;
    for v in held_card {
        if winning_nums.contains(&v) {
            num_of_winning += 1;
        }
    }

    if num_of_winning > 0 {
        return 2_i32.pow(num_of_winning - 1).try_into().unwrap();
    }
    return 0;
}

fn parse(s: &String) -> (HashSet<usize>, Vec<usize>) {
    let cards = s
        .split(":")
        .last()
        .unwrap()
        .split("|")
        .map(|s| s.trim().to_string().replace("  ", " "))
        .map(|s| {
            return s
                .split(" ")
                .map(|cards| cards.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        })
        .collect::<Vec<Vec<usize>>>();

    let winning_nums = make_set(cards.get(0).unwrap().to_vec());
    let held = cards.get(1).unwrap().to_vec();
    return (winning_nums, held);
}

fn compute_points(cb: impl Fn(Vec<usize>, HashSet<usize>) -> usize) -> impl Fn(&String) -> usize {
    move |s: &String| {
        let (winning_nums, held) = parse(s);
        return cb(held, winning_nums);
    }
}

fn into_card(held_card: Vec<usize>, winning_nums: HashSet<usize>) -> usize {
    let mut num_of_winning = 0;
    for v in held_card {
        if winning_nums.contains(&v) {
            num_of_winning += 1;
        }
    }

    if num_of_winning > 0 {
        return 2_i32.pow(num_of_winning - 1).try_into().unwrap();
    }
    return 0;
}

fn process_part2(data: Vec<String>) -> usize {

    let mut cards: Vec<_> = data
        .iter()
        .map(|s| {
            let (winning_nums, held) = parse(s);
            return Card {
                instances: 1,
                winning_nums,
                held,
            };
        })
    .collect();

    let mut total: usize = 0;

    for i in 0..cards.len() {
        let card = cards.get(i).unwrap();
        let instances = card.instances;
        total += instances;
        let mut num_of_winning = 0;
        for v in card.held.iter() {
            if card.winning_nums.contains(&v) {
                num_of_winning += 1;
            }
        }

        if num_of_winning > 0 {
            for j in (i+1)..(i+num_of_winning+1) {
                if let Some(c) = cards.get_mut(j) {
                    c.inc(instances);
                };
            }
        }
    }

    return total;
}

pub fn execute(data: Vec<String>) {
    let result: usize = data
        .clone()
        .iter()
        .map(compute_points(&part1_points_calculator))
        .sum();

    dbg!(result);


    let result2: usize = process_part2(data);

    dbg!(result2);

}
