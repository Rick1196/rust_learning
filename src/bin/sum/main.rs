fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut needed_numbers: HashMap<i32, i32> = HashMap::new();
    let mut results: Vec<i32> = vec![];
    use std::collections::HashMap;
    for (index, number) in nums.iter().enumerate() {
        let needed = target - number;
        if needed_numbers.contains_key(&needed) {
            results.push(index as i32);
            results.push(*needed_numbers.get(&needed).unwrap());
        } else {
            needed_numbers.insert(*number, index as i32);
        }
    }
    results
}

fn main() {
    let nums = vec![2, 11, 7, 15];
    let results = two_sum(nums, 9);
    println!("{:?}", results);
}
