pub fn state_and_operations(data: Vec<String>) -> (Vec<Vec<char>>, Vec<String>) {
    let i = data.iter().position(|r| r.as_str() == "").unwrap();
    let (state, ops) = data.split_at(i);

    let num_of_stacks = (state.get(0).unwrap().len() / 4) + 1;
    let mut state_of_stacks: Vec<Vec<char>> = Vec::with_capacity(num_of_stacks);

    for _ in 0..num_of_stacks {
        state_of_stacks.push(Vec::new());
    }

    let (_, stacks_temp) = state.split_last().unwrap(); 
    let mut stacks = stacks_temp.to_owned();
    stacks.reverse();
     
    for s in stacks.iter() {
        for (i, c) in s.char_indices() {
            match c {
                ' ' => continue,
                '[' => continue,
                ']' => continue,
                _ => {
                    let stack_i = i / 4;
                    state_of_stacks[stack_i].push(c);
                },

            } 
        }
    }

    let (_, operations) = ops.split_first().unwrap();
    return (state_of_stacks, operations.to_owned());
}

pub fn execute(data: Vec<String>) {
    let (mut state, ops) = state_and_operations(data); 
    for op in ops.iter() {
        let spl = op.split(" ").collect::<Vec<&str>>();
        let num_moved = spl[1].parse::<i32>().unwrap();
        let from = spl[3].parse::<usize>().unwrap() - 1;
        let to = spl[5].parse::<usize>().unwrap() - 1;

        // part1
        // for _ in 0..num_moved {
            // let popped = state[from].pop().expect("could not pop from stack");
            // state[to].push(popped);
        // }


        // part2 - lazy
        let mut temp: Vec<char> = vec![];
        for _ in 0..num_moved {
            let popped = state[from].pop().expect("could not pop from stack");
            temp.push(popped);
        }

        for _ in 0..num_moved {
            let popped = temp.pop().expect("could not pop from stack");
            state[to].push(popped);
        }
    }

    // get top items from stacks
    let result = state.iter_mut()
        .fold(String::from(""), |acc, stack| {
            return format!("{}{}", acc, stack.pop().unwrap());
        });

    dbg!(result);
}

