fn find_smallest_number(nums: &mut [i32], starting_from: usize) -> usize {
    let mut index_smallest_number = starting_from;
    let n = nums.len();
    for index in starting_from..n - 1 {
        if nums[index] < nums[index_smallest_number] {
            index_smallest_number = index;
        }
    }
    println!("smalles in {:?} {:?}", nums, index_smallest_number);
    index_smallest_number
}

fn selection_sort(nums: &mut [i32]) -> &mut [i32] {
    let n = nums.len();
    for index in 0..n - 1 {
        let index_smallest_number = find_smallest_number(nums, index);
        nums.swap(index, index_smallest_number);
        println!("{:?}", nums);
    }
    nums
}

fn main() {
    let unsorted_array = &mut [5, 10, 3, 1, 16];
    let result = selection_sort(unsorted_array);
    println!("sorted array, {:?}", result);
}
