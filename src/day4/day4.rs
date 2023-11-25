fn get_min_and_max(s: &str) -> (i32, i32) {
    let l = s.split("-").collect::<Vec<&str>>()[0];
    let r = s.split("-").collect::<Vec<&str>>()[1];
    return (l.to_string().parse().unwrap(), r.to_string().parse().unwrap());
}
pub fn execute(data: Vec<String>) {
    let result = data.clone().iter()
        .fold(0, |acc, pairs| {

            let l = pairs.split(",").collect::<Vec<&str>>()[0];
            let r = pairs.split(",").collect::<Vec<&str>>()[1];
            
            let (l_min, l_max) = get_min_and_max(l);
            let (r_min, r_max) = get_min_and_max(r);

            if r_min >= l_min && r_max <= l_max {
                return acc + 1;
            }

            if l_min >= r_min && l_max <= r_max {
                return acc + 1;
            }
            
            return acc;
        });

    dbg!(result);

    let result_2 = data.iter()
        .fold(0, |acc, pairs| {

            let l = pairs.split(",").collect::<Vec<&str>>()[0];
            let r = pairs.split(",").collect::<Vec<&str>>()[1];
            
            let (l_min, l_max) = get_min_and_max(l);
            let (r_min, r_max) = get_min_and_max(r);

            // check if l_min overlaps
            if l_min >= r_min && l_min <= r_max {
                return acc + 1;
            }            
            // check if l_max overlaps
            if l_max >= r_min && l_max <= r_max {
                return acc + 1;
            }            
            // check if r_min overlaps
            if r_min >= l_min && r_min <= l_max {
                return acc + 1;
            }
            // check if r_max overlaps
            if r_max >= l_min && r_max <= l_max {
                return acc + 1;
            }

            return acc;
        });

    dbg!(result_2);

}

