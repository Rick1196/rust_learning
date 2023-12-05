use rand::Rng;

fn merge(nums: &mut Vec<i64>, left_limit: usize, right_limit: usize, mid: usize) {
    let mut temp: Vec<_> = Vec::with_capacity(right_limit - left_limit + 1);
    let mut i = left_limit;
    let mut j = mid + 1;
    while i <= mid && j <= right_limit {
        if nums[i] <= nums[j] {
            temp.push(nums[i]);
            i += 1;
        } else {
            temp.push(nums[j]);
            j += 1;
        }
    }

    while i <= mid {
        temp.push(nums[i]);
        i += 1;
    }
    while j <= right_limit {
        temp.push(nums[j]);
        j += 1;
    }
    nums.splice(left_limit..right_limit + 1, temp);
}

fn merge_sort(nums: &mut Vec<i64>, left_limit: usize, right_limit: usize) {
    if right_limit - left_limit < 1 {
        return;
    }
    let mid = (left_limit + right_limit) / 2;
    merge_sort(nums, left_limit, mid);
    merge_sort(nums, mid + 1, right_limit);
    merge(nums, left_limit, right_limit, mid);
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut numbers = Vec::<i64>::with_capacity(10);
    for _ in 0..10 {
        numbers.push(rng.gen_range(1..1000001));
    }
    // println!("unsorted {:?}", numbers);
    merge_sort(&mut numbers, 0, 10 - 1);
    println!("sorted array, {:?}", numbers);
}
