fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_sum = std::i32::MIN;
    let mut current_sum = 0;

    for &num in arr {
        current_sum = current_sum + num;
        if num > current_sum {
            current_sum = num;
        }
        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }

    max_sum
}

fn main() {
    let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);
    println!("Maximum subarray sum: {}", max_sum);
}
