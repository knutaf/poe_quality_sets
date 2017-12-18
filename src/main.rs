use std::env;

fn find_best_combo(orig_target : u32, max_possible_sets : u32, target : u32, num_sets : u32, num_pending : u32, nums : &Vec<u32>) -> (Vec<u32>, u32, u32) {
    if !nums.is_empty() {
        let mut best_set = false;
        let mut best_num_sets : u32 = num_sets;
        let mut best_num_pending : u32 = num_pending;
        let mut best_consumed_num : u32 = nums[0];
        let mut best_sub_combo : Vec<u32> = vec![];

        for (consumed_index, consumed_num) in nums.iter().enumerate() {
            if best_num_sets < max_possible_sets {
                let (new_target, new_num_sets, new_num_pending) = if *consumed_num < target {
                    (target - *consumed_num, num_sets, num_pending + 1)
                } else if *consumed_num == target {
                    (orig_target, num_sets + 1, 0)
                } else {
                    (orig_target, num_sets, num_pending + 1)
                };

                if *consumed_num <= target {
                    let remaining_nums : Vec<u32> = nums.iter().enumerate().filter(|&(n, _)| { n != consumed_index }).map(|(_, not_consumed_num)| { *not_consumed_num }).collect();

                    //println!("consumed {}, remaining {:?}", consumed_num, remaining_nums);

                    let (sub_combo, sub_num_sets, sub_num_pending) = find_best_combo(orig_target, max_possible_sets, new_target, new_num_sets, new_num_pending, &remaining_nums);

                    if orig_target == target && num_sets == 0 {
                        println!("consumed {}, sub_combo {:?}, sub_num_sets {}, sub_num_pending {}", *consumed_num, sub_combo, sub_num_sets, sub_num_pending);
                    }

                    if !best_set ||
                       sub_num_sets > best_num_sets ||
                       (sub_num_sets == best_num_sets && sub_num_pending < best_num_pending) {
                        best_set = true;
                        best_num_sets = sub_num_sets;
                        best_num_pending = sub_num_pending;
                        best_consumed_num = *consumed_num;
                        best_sub_combo = sub_combo;
                    }
                } else {
                    best_num_pending = new_num_pending;
                }
            }
        }

        let mut combo : Vec<u32> = vec![best_consumed_num];
        combo.extend(best_sub_combo.iter().map(|x| { *x }));
        let ret = (combo, best_num_sets, best_num_pending);
        //println!("ret: {:?}", ret);
        ret
    } else {
        (nums.clone(), num_sets, num_pending)
    }
}

#[derive(Debug)]
struct SetInfo {
    items : Vec<u32>,
    num_sets : u32,
    num_remaining : u32,
}

fn get_best_set(target : u32, nums : Vec<u32>) -> SetInfo {
    let max_possible_sets : u32 = nums.iter().cloned().fold(0, |acc, x| { acc + x }) / target;
    println!("max sets possible: {}", max_possible_sets);

    let best_combo = find_best_combo(target, max_possible_sets, target, 0, 0, &nums);
    SetInfo {
        items : best_combo.0,
        num_sets : best_combo.1,
        num_remaining : best_combo.2,
    }
}

fn main() {
    let nums : Vec<u32> = env::args().skip(1).map(|s| {
        s.trim().parse::<u32>().expect("failed to parse")
    }).collect();

    //println!("nums: {:?}", nums);

    if !nums.is_empty() {
        let best_set = get_best_set(40, nums);
        println!("best set: {:?}", best_set);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_best_set_counts(target : u32, nums : Vec<u32>) -> (u32, u32) {
        let set = get_best_set(target, nums);
        (set.num_sets, set.num_remaining)
    }

    #[test]
    fn a01() {
        assert_eq!(get_best_set_counts(10, vec![]), (0, 0));
    }

    #[test]
    fn a02() {
        assert_eq!(get_best_set_counts(10, vec![1]), (0, 1));
    }

    #[test]
    fn a03() {
        assert_eq!(get_best_set_counts(10, vec![10]), (1, 0));
    }

    #[test]
    fn a04() {
        assert_eq!(get_best_set_counts(10, vec![5, 5]), (1, 0));
    }

    #[test]
    fn a05() {
        assert_eq!(get_best_set_counts(10, vec![5, 6]), (0, 2));
    }

    #[test]
    fn a06() {
        assert_eq!(get_best_set_counts(10, vec![5, 5, 1]), (1, 1));
    }

    #[test]
    fn a07() {
        assert_eq!(get_best_set_counts(10, vec![5, 4]), (0, 2));
    }

    #[test]
    fn a08() {
        assert_eq!(get_best_set_counts(10, vec![5, 1, 5]), (1, 1));
    }

    #[test]
    fn a09() {
        assert_eq!(get_best_set_counts(10, vec![1, 5, 5]), (1, 1));
    }

    #[test]
    fn a10() {
        assert_eq!(get_best_set_counts(10, vec![8, 2, 2, 2, 2, 2]), (1, 1));
    }

    #[test]
    fn a11() {
        assert_eq!(get_best_set_counts(10, vec![5, 5, 5, 5]), (2, 0));
    }

    #[test]
    fn a12() {
        assert_eq!(get_best_set_counts(10, vec![5, 5, 5, 5, 1]), (2, 1));
    }
}
