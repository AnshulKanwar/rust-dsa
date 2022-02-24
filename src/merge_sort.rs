pub fn merge_sort<T: PartialOrd + Copy>(arr: &mut [T], p: usize, r: usize) {
    if p >= r {
        return;
    }

    let q = (p + r) / 2;
    merge_sort(arr, p, q);
    merge_sort(arr, q+1, r);
    let merged_vec = merge(&arr[p..q+1], &arr[q+1..r+1]);

    let mut k = p;
    for i in &merged_vec {
        arr[k] = i.clone();
        k += 1;
    }
}

fn merge<T: PartialOrd + Copy>(left: &[T], right: &[T]) -> Vec<T> {
    let mut i = 0;
    let mut j = 0;

    let mut merged_array = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged_array.push(left[i]);
            i += 1;
        } else {
            merged_array.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        merged_array.extend_from_slice(&left[i..]);
    }
    if j < right.len() {
        merged_array.extend_from_slice(&right[j..]);
    }

    merged_array
}

#[cfg(test)]
mod tests {
    use super::*;

    // For merge_sort()
    #[test]
    fn sort_array() {
        let mut arr = [1,5,2,7,9];
        let len = arr.len();
        merge_sort(&mut arr, 0, len - 1);
        assert_eq!(arr, [1,2,5,7,9]);

        let mut arr = [7,3,97,23,12,4,65,90];
        let len = arr.len();
        merge_sort(&mut arr, 0, len - 1);
        assert_eq!(arr, [3,4,7,12,23,65,90,97]);

        let mut arr = [69];
        let len = arr.len();
        merge_sort(&mut arr, 0, len - 1);
        assert_eq!(arr, [69]);
    }

    #[test]
    fn sort_ascending_array() {
        let mut arr = [1,2,3,4,5,6];
        let len = arr.len();
        merge_sort(&mut arr, 0, len - 1);
        assert_eq!(arr, [1,2,3,4,5,6]);
    }

    #[test]
    fn sort_descending_array() {
        let mut arr = [6,5,4,3,2,1];
        let len = arr.len();
        merge_sort(&mut arr, 0, len - 1);
        assert_eq!(arr, [1,2,3,4,5,6]);
    }

    // For merge()
    #[test]
    fn merge_left_then_right() {
        let left = [1, 2, 3];
        let right = [4, 5, 6];
        assert_eq!(merge(&left, &right), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_right_then_left() {
        let left = [4, 5, 6];
        let right = [1, 2, 3];
        assert_eq!(merge(&left, &right), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn merge_mix() {
        let left = [2, 9];
        let right = [5];
        assert_eq!(merge(&left, &right), [2, 5, 9]);
    }

    #[test]
    fn one_array_empty() {
        let empty: [i32; 0] = [];
        let non_empty = [1, 2, 3];
        assert_eq!(merge(&empty, &non_empty), [1, 2, 3]);
        assert_eq!(merge(&empty, &non_empty), [1, 2, 3]);
    }
}