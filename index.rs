fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut result = None;

    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            result = Some(mid);
            right = mid - 1; // Move to the left to find the first occurrence
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    result
}

fn main() {
    let arr = [1, 2, 3, 3, 3, 4, 5, 6, 7];
    let target = 3;
    
    if let Some(index) = find_first_occurrence(&arr, target) {
        println!("The first occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in the array", target);
    }
}
