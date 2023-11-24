use std::collections::BinaryHeap;
pub fn execute(data: Vec<String>) {

    let mut heap = BinaryHeap::new();

    let mut current_acc = 0;
    for val in data {
        match val.len() {
            0 => {

                heap.push(current_acc);
                current_acc = 0;
            },
            _ => {
                let v: i32 = val.parse().expect("could not parse");
                current_acc += v;
            },
        }
    }

    let result = heap.pop().unwrap()
        + heap.pop().unwrap()
        + heap.pop().unwrap();

    dbg!(result);
}
