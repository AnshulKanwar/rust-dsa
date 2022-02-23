pub fn binary_search<T: PartialOrd>(arr: &[T], ele: T) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = (low + high) / 2;
        if ele < arr[mid] {
            high = mid - 1;
        } else if ele > arr[mid] {
            low = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn element_present() {
        let arr = [2,4,6,8,9];
        assert_eq!(binary_search(&arr, 2), Some(2));
        assert_eq!(binary_search(&arr, 6), Some(2));
        assert_eq!(binary_search(&arr, 9), Some(2));
    }

    #[test]
    fn element_not_present() {
        let arr = [2,4,6,8,9];
        assert_eq!(binary_search(&arr, 1), None);
        assert_eq!(binary_search(&arr, 5), None);
        assert_eq!(binary_search(&arr, 10), None);
    }
}