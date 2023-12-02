#[derive(Debug)]
pub struct Game {
    sets: Vec<GameSet>,
    id: usize,
}

#[derive(Debug)]
pub struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameSet {
    fn construct(s: Vec<&str>) -> GameSet {
        let mut gs = GameSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        for color_count in s.iter() {
            let (num_str, color) = color_count.split_once(" ").unwrap();
            let num: usize = num_str.parse().unwrap();
            match color {
                "red" => gs.red = num,
                "blue" => gs.blue = num,
                "green" => gs.green = num,
                _ => panic!("color invalid {:?}", color),
            }
        }

        return gs;
    }
}

fn multiply_fewest_possible_cubes(game: Game) -> usize {
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    for set in game.sets.iter() {
        max_red = Ord::max(max_red, set.red);
        max_blue = Ord::max(max_blue, set.blue);
        max_green = Ord::max(max_green, set.green);
    }

    let prod = max_red * max_blue * max_green;
    return prod;
}

fn return_game_id_if_game_possible(game: Game) -> usize {
    for set in game.sets.iter() {
        if set.red > 12 {
            return 0;
        }
        if set.green > 13 {
            return 0;
        }
        if set.blue > 14 {
            return 0;
        }
    }

    return game.id;
}

pub fn process(data: Vec<String>, cb: &dyn Fn(Game) -> usize) {
    let result: usize = data
        .iter()
        .map(|game| {
            let game_id = game.split(":").collect::<Vec<&str>>()[0]
                .split(" ")
                .collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            let sets = game
                .split(":")
                .last()
                .unwrap()
                .trim()
                .split(";")
                .map(|s| s.trim())
                .map(|s| {
                    let game_set_string = s
                        .split(",")
                        .map(|count| count.trim())
                        .collect::<Vec<&str>>();
                    return game_set_string;
                })
                .map(GameSet::construct)
                .collect::<Vec<GameSet>>();

            return Game { id: game_id, sets };
        })
        .map(cb)
        .sum();

    dbg!(result);
}

pub fn execute(data: Vec<String>) {
    process(data.clone(), &return_game_id_if_game_possible);
    process(data, &multiply_fewest_possible_cubes);
}
