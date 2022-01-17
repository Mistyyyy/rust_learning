use std::collections::HashMap;


#[derive(Debug)]
pub struct OpResult {
    pub average: usize,
    pub mode: usize,
    pub middle: usize
}

fn get_average(list: &Vec<usize>) -> usize {
    let len = list.len();
    let mut sum = 0;
    for i in list {
        sum += i;
    }

    sum / len
}

fn get_mode(list: &Vec<usize>) -> usize {
    let mut max_count: usize = 0;
    let mut mode_count: &usize = &0;
    let mut counter_maps: HashMap<&usize, usize> = HashMap::new();

    for i in list {
        let count = counter_maps.entry(i).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode_count = i;
        }
    }
    *mode_count
}

fn get_middle(list: &Vec<usize>) -> usize {
    let mut vesc = list.clone();
    vesc.sort();
    let len = vesc.len();
    match vesc.get(len / 2) {
        Some(value) => *value,
        None => 0
    }
}

pub fn op_number_vec(list: &Vec<usize>) -> OpResult {
    let average = get_average(list);
    let mode = get_mode(list);
    let middle = get_middle(list);

    OpResult {
        average,
        mode,
        middle
    }
} 