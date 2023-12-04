use std::collections::HashMap;

use regex::Regex;

fn border_has_symbol(coord: Vec<(usize, usize)>, m: &Vec<Vec<char>>) -> bool {
    let r = Regex::new("[^A-Za-z0-9.]").unwrap();
    for c in coord {
        let (i, j) = c;

        if i > 0 {
            match m.get(i - 1) {
                Some(s) => {
                    let v = s.get(j).unwrap();
                    if r.captures(&v.to_string()).is_some() {
                        return true;
                    }
                    if j > 0 {
                        match s.get(j - 1) {
                            Some(v) => {
                                if r.captures(&v.to_string()).is_some() {
                                    return true;
                                }
                            }
                            None => {}
                        }
                    }
                    match s.get(j + 1) {
                        Some(v) => {
                            if r.captures(&v.to_string()).is_some() {
                                return true;
                            }
                        }
                        None => {}
                    }
                }
                None => {dbg!("none");}
            }

        }
        match m.get(i + 1) {
            Some(s) => {
                let v = s.get(j).unwrap();
                if r.captures(&v.to_string()).is_some() {
                    return true;
                }
                if j > 0 {
                    match s.get(j - 1) {
                        Some(v) => {
                            if r.captures(&v.to_string()).is_some() {
                                return true;
                            }
                        }
                        None => {}
                    }
                }
                match s.get(j + 1) {
                    Some(v) => {
                        if r.captures(&v.to_string()).is_some() {
                            return true;
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }

        let slice = m.get(i).unwrap();
        if j > 0 {
            match slice.get(j - 1) {
                Some(v) => {
                    if r.captures(&v.to_string()).is_some() {
                        return true;
                    }
                }
                None => {}
            }
        }
        match slice.get(j + 1) {
            Some(v) => {
                if r.captures(&v.to_string()).is_some() {
                    return true;
                }
            }
            None => {}
        }
    }

    return false;
}

fn part1(matrix: Vec<Vec<char>>) {
    let mut temp_digit_str = String::from("");

    let mut coords: Vec<(usize, usize)> = vec![];

    let mut total = 0;

    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c.to_string().parse::<usize>() {
                Ok(d) => {
                    temp_digit_str = format!("{}{}", temp_digit_str, d.to_string());
                    coords.push((i, j));
                }
                Err(_) => {
                    if border_has_symbol(coords, &matrix) {
                        total += temp_digit_str.parse::<usize>().unwrap();
                    }

                    temp_digit_str = String::from("");

                    //clear coords
                    coords = vec![];
                }
            }
        }
    }

    dbg!(total);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct NumberLocation {
    start: (usize, usize),
    end: (usize, usize),
}

fn is_valid_gear(i: usize, j: usize, m: &Vec<Vec<char>>) {
    let r = Regex::new("[0-9]").unwrap();
    if i > 0 {
        match m.get(i - 1) {
            Some(s) => {
                let v = s.get(j).unwrap();
                if r.captures(&v.to_string()).is_some() {
                    dbg!(v);

                }
                if j > 0 {
                    match s.get(j - 1) {
                        Some(v) => {
                            if r.captures(&v.to_string()).is_some() {
                                dbg!(v);

                            }
                        }
                        None => {}
                    }
                }
                match s.get(j + 1) {
                    Some(v) => {
                        if r.captures(&v.to_string()).is_some() {

                            dbg!(v);
                        }
                    }
                    None => {}
                }
            }
            None => {dbg!("none");}
        }

    }
    match m.get(i + 1) {
        Some(s) => {
            let v = s.get(j).unwrap();
            if r.captures(&v.to_string()).is_some() {

                dbg!(v);
            }
            if j > 0 {
                match s.get(j - 1) {
                    Some(v) => {
                        if r.captures(&v.to_string()).is_some() {

                            dbg!(v);
                        }
                    }
                    None => {}
                }
            }
            match s.get(j + 1) {
                Some(v) => {
                    if r.captures(&v.to_string()).is_some() {

                        dbg!(v);
                    }
                }
                None => {}
            }
        }
        None => {}
    }

    let slice = m.get(i).unwrap();
    if j > 0 {
        match slice.get(j - 1) {
            Some(v) => {
                if r.captures(&v.to_string()).is_some() {

                    dbg!(v);
                }
            }
            None => {}
        }
    }
    match slice.get(j + 1) {
        Some(v) => {
            if r.captures(&v.to_string()).is_some() {

                dbg!(v);
            }
        }
        None => {}
    }
}

fn part2(matrix: Vec<Vec<char>>) {
    let mut temp_digit_str = String::from("");

    let mut gear_coords: Vec<(usize, usize)> = vec![];

    let mut num_locations: HashMap<NumberLocation, String> = HashMap::new();
    let mut coords: Vec<(usize, usize)> = vec![];


    for (i, row) in matrix.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'*' {
                gear_coords.push((i, j));

                is_valid_gear(i, j, &matrix);
            }

            match c.to_string().parse::<usize>() {
                Ok(d) => {
                    temp_digit_str = format!("{}{}", temp_digit_str, d.to_string());
                    coords.push((i, j));
                }
                Err(_) => {
                    if coords.len() > 0 {
                        let start = coords.first().unwrap().to_owned();
                        let end = coords.last().unwrap().to_owned();
                        num_locations.insert(NumberLocation {start, end}, temp_digit_str.clone());
                    }

                    // clear digit
                    temp_digit_str = String::from("");
                    //clear coords
                    coords = vec![];
                }
            }
        }
    }

    dbg!(coords);
}
pub fn execute(data: Vec<String>) {
    let matrix: Vec<_> = data
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    // part1(matrix.clone());
    part2(matrix);
}

