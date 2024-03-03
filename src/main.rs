use std::{
    collections::{HashMap, HashSet},
    io,
};

fn solution(
    building_time: &Vec<u32>,
    path: &HashMap<usize, HashSet<usize>>,
    target_index: usize,
) -> u32 {
    let mut building_offset: Vec<Option<u32>> = vec![None; building_time.len()];
    loop {
        if let Some(result) = building_offset[target_index] {
            return result;
        }
        building_offset = building_offset
            .iter()
            .enumerate()
            .map(|(index, &option)| {
                if option != None {
                    return option;
                }
                if let Some(set) = path.get(&index) {
                    if set.iter().all(|&index| building_offset[index] != None) {
                        Some(
                            set.iter()
                                .map(|&index| building_offset[index].unwrap())
                                .fold(0, |acc, curr| acc.max(curr))
                                + building_time[index],
                        )
                    } else {
                        None
                    }
                } else {
                    Some(building_time[index])
                }
            })
            .collect()
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split(' ').flat_map(|x| x.parse::<usize>());
        iter.next();
        let path_count = iter.next().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let building_time = line
            .trim()
            .split(' ')
            .flat_map(|x| x.parse::<u32>())
            .collect::<Vec<_>>();
        let mut path: HashMap<usize, HashSet<usize>> = HashMap::new();
        for _ in 0..path_count {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.trim().split(' ').flat_map(|x| x.parse::<usize>());
            let precondition = iter.next().unwrap() - 1;
            let target = iter.next().unwrap() - 1;
            if let Some(set) = path.get_mut(&target) {
                set.insert(precondition);
            } else {
                let mut set = HashSet::new();
                set.insert(precondition);
                path.insert(target, set);
            }
        }
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let target_index = line.trim().parse::<usize>().unwrap() - 1;

        println!("{}", solution(&building_time, &path, target_index));
    }
}
