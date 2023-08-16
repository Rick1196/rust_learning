fn insertion_sort(nums: &mut [i32]) -> &mut [i32] {
    let n = nums.len();
    for index in 0..n - 1 {
        let mut sup_index = index;
        while sup_index < nums.len() {
            if nums[index] > nums[sup_index] {
                nums.swap(index, sup_index);
                println!("{:?}", nums)
            }
            sup_index = sup_index + 1;
        }
    }
    nums
}

fn main() {
    let unsorted_array = &mut [5, 10, 3, 1, 16];
    let result = insertion_sort(unsorted_array);
    println!("sorted array, {:?}", result);
}
