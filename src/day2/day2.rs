use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Move {
    A(Rock),
    B(Paper),
    C(Scissors),
    X(Rock),
    Y(Paper),
    Z(Scissors),
}

enum Type {
    X,
    Y,
    Z,
}

impl FromStr for Type {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "X" => Ok(Type::X),
            "Y" => Ok(Type::Y),
            "Z" => Ok(Type::Z),
            _ => Err(()),
        }
    }
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(Move::A(Rock::default())),
            "B" => Ok(Move::B(Paper::default())),
            "C" => Ok(Move::C(Scissors::default())),
            "X" => Ok(Move::X(Rock::default())),
            "Y" => Ok(Move::Y(Paper::default())),
            "Z" => Ok(Move::Z(Scissors::default())),
            _ => Err(()),
        }
    }
}

impl Move {
    pub fn score(opp: Move, me: Move) -> i32 {
        match opp {
            Move::A(_) => match me {
                Move::X(m) => {
                    return 3 + m.val;
                }
                Move::Y(m) => {
                    return 6 + m.val;
                }
                Move::Z(m) => {
                    return 0 + m.val;
                }
                _ => {}
            },
            Move::B(_) => match me {
                Move::X(m) => {
                    return 0 + m.val;
                }
                Move::Y(m) => {
                    return 3 + m.val;
                }
                Move::Z(m) => {
                    return 6 + m.val;
                }
                _ => {}
            },
            Move::C(_) => match me {
                Move::X(m) => {
                    return 6 + m.val;
                }
                Move::Y(m) => {
                    return 0 + m.val;
                }
                Move::Z(m) => {
                    return 3 + m.val;
                }
                _ => {}
            },
            _ => {}
        }
        return 0;
    }
}

#[derive(Debug, PartialEq)]
struct Rock {
    val: i32,
}

impl Default for Rock {
    fn default() -> Rock {
        Rock { val: 1 }
    }
}

#[derive(Debug, PartialEq)]
struct Paper {
    val: i32,
}

impl Default for Paper {
    fn default() -> Paper {
        Paper { val: 2 }
    }
}

#[derive(Debug, PartialEq)]
struct Scissors {
    val: i32,
}

impl Default for Scissors {
    fn default() -> Scissors {
        Scissors { val: 3 }
    }
}

fn get_me(opp: &Move, t: Type) -> Move {
    match opp {
        &Move::A(_) => match t {
            Type::X => Move::Z(Scissors::default()),
            Type::Y => Move::X(Rock::default()),
            Type::Z => Move::Y(Paper::default()),
        },
        &Move::B(_) => match t {
            Type::X => Move::X(Rock::default()),
            Type::Y => Move::Y(Paper::default()),
            Type::Z => Move::Z(Scissors::default()),
        },
        &Move::C(_) => match t {
            Type::X => Move::Y(Paper::default()),
            Type::Y => Move::Z(Scissors::default()),
            Type::Z => Move::X(Rock::default()),
        },
        _ => Move::Z(Scissors::default()),
    }
}

pub fn execute(data: Vec<String>) {
    let mut score = 0;
    for ele in data.clone() {
        let v: Vec<&str> = ele.split(" ").collect();
        let opp = Move::from_str(v.get(0).unwrap()).expect("could not convert to enum");
        let me = Move::from_str(v.get(1).unwrap()).expect("could not convert to enum");
        score += Move::score(opp, me);
    }
    dbg!("Part1:{:?}", score);

    let mut part2_score = 0;
    for ele in data {
        let v: Vec<&str> = ele.split(" ").collect();
        let opp = Move::from_str(v.get(0).unwrap()).expect("could not convert to enum");
        let t = Type::from_str(v.get(1).unwrap()).expect("could not convert to enum");
        let me = get_me(&opp, t);
        part2_score += Move::score(opp, me);
    }

    dbg!(part2_score);
}
