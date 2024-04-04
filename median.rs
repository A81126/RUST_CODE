fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // If the length of the array is even
        let mid_index1 = (len - 1) / 2;
        let mid_index2 = mid_index1 + 1;
        return (arr[mid_index1] as f64 + arr[mid_index2] as f64) / 2.0;
    } else {
        // If the length of the array is odd
        let mid_index = len / 2;
        return arr[mid_index] as f64;
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7];
    println!("Median of the array: {}", find_median(&arr));

   
}