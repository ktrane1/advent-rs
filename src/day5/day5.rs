#[derive(Debug, Clone)]
struct Range {
    source_start: usize,
    destination_start: usize,
    len: usize,
}

#[derive(Debug, Clone)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}

fn get_seeds(s: String) -> Vec<usize> {
    s.replace("seeds: ", "")
        .split(" ")
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn get_rest(strings: Vec<String>) -> Vec<Map> {
    let mut result: Vec<Map> = vec![];
    let mut peeky = strings.iter().peekable();
    // first is empty string, so skip it!
    peeky.next();

    let mut m: Map = Map {
        source: "".to_string(),
        destination: "".to_string(),
        ranges: vec![],
    };

    loop {
        let s = peeky.next();
        if s.unwrap().is_empty() {
            result.push(m.clone());
            m.ranges = vec![];
        }

        if let Some(first_char) = s.unwrap().chars().next() {
            if first_char.is_digit(10) {
                let range_string = s.unwrap();
                let range_values = range_string
                    .split(" ")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();

                m.ranges.push(Range {
                    destination_start: range_values[0],
                    source_start: range_values[1],
                    len: range_values[2],
                });
            } else {
                let map_string = s.unwrap();
                let split = map_string.split(" ").collect::<Vec<&str>>();
                let (text, _) = split.split_first().unwrap();
                let replaced = text.replace("-to-", " ");
                let source_to_destination = replaced.split(" ").collect::<Vec<&str>>();

                m.source = source_to_destination[0].to_string();
                m.destination = source_to_destination[1].to_string();
            }
        }

        if peeky.peek().is_none() {
            result.push(m.clone());
            m.ranges = vec![];
            break;
        }
    }

    return result;
}

fn construct_seeds_and_maps(data: Vec<String>) -> (Vec<usize>, Vec<Map>) {
    let all_data = data.iter().map(String::from).collect::<Vec<String>>();

    let (seeds_raw, rest_raw) = all_data.split_first().unwrap();

    let seeds = get_seeds(seeds_raw.to_string());
    let maps = get_rest(rest_raw.to_vec());
    return (seeds, maps);
}

fn map_seed_to_location(seeds: Vec<usize>, maps: Vec<Map>) -> Vec<usize> {
    let mut result = vec![];
    for seed in seeds {
        let mut mapped_seed = seed;
        for m in &maps {
            let ranges = &m.ranges;
            for r in ranges {
                if mapped_seed >= r.source_start && mapped_seed <= r.source_start + r.len {
                    let diff = mapped_seed - r.source_start;
                    mapped_seed = r.destination_start + diff;
                    break;
                }
            }
        }
        result.push(mapped_seed);
    }
    result.sort();
    return result;
}

pub fn execute(data: Vec<String>) {
    let (seeds, maps) = construct_seeds_and_maps(data);
    let result = map_seed_to_location(seeds, maps)[0];
    dbg!(result);
}
