fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

fn main() {
    let arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
    match kth_smallest(&arr, k) {
        Some(val) => println!("The {}th smallest element is: {}", k, val),
        None => println!("Invalid input: k is larger than the array length"),
    }
}
