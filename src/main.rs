use std::env;

fn find_good_ordering(orig_target : u32, max_possible_sets : u32, target : u32, num_sets : u32, nums : &Vec<u32>) -> (Vec<u32>, u32) {
    if !nums.is_empty() {
        let mut best_set = false;
        let mut best_num_sets : u32 = num_sets;
        let mut best_consumed_num : u32 = nums[0];
        let mut best_sub_combo : Vec<u32> = vec![];

        for (consumed_index, consumed_num) in nums.iter().enumerate() {
            if best_num_sets < max_possible_sets {
                let (new_target, new_num_sets) = if *consumed_num < target {
                    (target - *consumed_num, num_sets)
                } else if *consumed_num == target {
                    (orig_target, num_sets + 1)
                } else {
                    (orig_target, num_sets)
                };

                if *consumed_num <= target {
                    let remaining_nums : Vec<u32> = nums.iter().enumerate().filter(|&(n, _)| { n != consumed_index }).map(|(_, not_consumed_num)| { *not_consumed_num }).collect();

                    //println!("consumed {}, remaining {:?}", consumed_num, remaining_nums);

                    let (sub_combo, sub_num_sets) = find_good_ordering(orig_target, max_possible_sets, new_target, new_num_sets, &remaining_nums);

                    /*
                    if orig_target == target && num_sets == 0 {
                        println!("consumed {}, sub_combo {:?}, sub_num_sets {}", *consumed_num, sub_combo, sub_num_sets);
                    }
                    */

                    if !best_set || sub_num_sets > best_num_sets {
                        best_set = true;
                        best_num_sets = sub_num_sets;
                        best_consumed_num = *consumed_num;
                        best_sub_combo = sub_combo;
                    }
                }
            }
        }

        let mut combo : Vec<u32> = vec![best_consumed_num];
        combo.extend(best_sub_combo.iter().map(|x| { *x }));
        let ret = (combo, best_num_sets);
        //println!("ret: {:?}", ret);
        ret
    } else {
        (nums.clone(), num_sets)
    }
}

fn get_good_sets(target : u32, nums : Vec<u32>) -> Vec<Vec<u32>> {
    let max_possible_sets : u32 = nums.iter().cloned().fold(0, |acc, x| { acc + x }) / target;
    println!("max sets possible: {}", max_possible_sets);

    let good_ordering = find_good_ordering(target, max_possible_sets, target, 0, &nums);

    let mut sets : Vec<Vec<u32>> = vec![];
    let _ = (good_ordering.0).iter().fold((0, vec![]), |(current_set_total, mut current_set), &item| {
        current_set.push(item);
        let new_total = current_set_total + item;

        if new_total == target {
            sets.push(current_set);
            (0, vec![])
        } else {
            (new_total, current_set)
        }
    });

    sets
}

fn main() {
    let nums : Vec<u32> = env::args().skip(1).map(|s| {
        s.trim().parse::<u32>().expect("failed to parse")
    }).collect();

    if !nums.is_empty() {
        println!("{:?}", get_good_sets(40, nums));
    } else {
        println!("Usage: {} quality_1 quality_2 quality_3 etc.", env::args().nth(0).expect("need exe name"));
        println!("  Input all your items' quality, and the tool prints out which order you should");
        println!("  sell them in, to get the most 40% quality sets possible without wasting any.");
        println!("");
        println!("  There may be some left over after, which you can just keep for next time.");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_good_set_count(target : u32, nums : Vec<u32>) -> u32 {
        let sets = get_good_sets(target, nums);
        sets.len() as u32
    }

    #[test]
    fn a01() {
        assert_eq!(get_good_set_count(10, vec![]), 0);
    }

    #[test]
    fn a02() {
        assert_eq!(get_good_set_count(10, vec![1]), 0);
    }

    #[test]
    fn a03() {
        assert_eq!(get_good_set_count(10, vec![10]), 1);
    }

    #[test]
    fn a04() {
        assert_eq!(get_good_set_count(10, vec![5, 5]), 1);
    }

    #[test]
    fn a05() {
        assert_eq!(get_good_set_count(10, vec![5, 6]), 0);
    }

    #[test]
    fn a06() {
        assert_eq!(get_good_set_count(10, vec![5, 5, 1]), 1);
    }

    #[test]
    fn a07() {
        assert_eq!(get_good_set_count(10, vec![5, 4]), 0);
    }

    #[test]
    fn a08() {
        assert_eq!(get_good_set_count(10, vec![5, 1, 5]), 1);
    }

    #[test]
    fn a09() {
        assert_eq!(get_good_set_count(10, vec![1, 5, 5]), 1);
    }

    #[test]
    fn a10() {
        assert_eq!(get_good_set_count(10, vec![8, 2, 2, 2, 2, 2]), 1);
    }

    #[test]
    fn a11() {
        assert_eq!(get_good_set_count(10, vec![5, 5, 5, 5]), 2);
    }

    #[test]
    fn a12() {
        assert_eq!(get_good_set_count(10, vec![5, 5, 5, 5, 1]), 2);
    }
}
