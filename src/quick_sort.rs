
pub fn quick_sort<T: PartialOrd + Copy>(mut arr: &mut [T], low: i32, high: i32) {
    if low >= high { return }
    let p = partition(&mut arr, 0, high as usize);
    quick_sort(&mut arr, low, (p as i32) - 1);
    quick_sort(&mut arr, (p as i32) + 1, high);
}

fn partition<T: PartialOrd + Copy>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = (low as i32) - 1;

    for j in low..high {
        if arr[j] < pivot {
            i += 1;
            arr.swap(i as usize, j);
        }
    }

    arr.swap((i+1) as usize, high);
    (i+1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_array() {
        let mut arr = [5,3,0,9,4,8];
        let len = arr.len();
        quick_sort(&mut arr, 0, (len - 1) as i32);
        assert_eq!(arr, [0,3,4,5,8,9])
    }

    #[test]
    fn sort_ascending_array() {
        let mut arr = [1,2,3,4,5,6];
        let len = arr.len();
        quick_sort(&mut arr, 0, (len - 1) as i32);
        assert_eq!(arr, [1,2,3,4,5,6])
    }

    #[test]
    fn sort_descending_array() {
        let mut arr = [6,5,4,3,2,1];
        let len = arr.len();
        quick_sort(&mut arr, 0, (len - 1) as i32);
        assert_eq!(arr, [1,2,3,4,5,6])
    }

    #[test]
    fn partition_normal() {
        let mut arr = [1,6,8,2,5,3,7,4];
        let len = arr.len();
        partition(&mut arr, 0, len - 1);
        assert_eq!(arr, [1, 2, 3, 4, 5, 8, 7, 6]);
    }

    #[test]
    fn partition_already_parititoned() {
        let mut arr = [1,4,2,5,9,10];
        let len = arr.len();
        partition(&mut arr, 0, len - 1);
        assert_eq!(arr, [1,4,2,5,9,10]);
    }

    #[test]
    fn partition_not_parititoned() {
        let mut arr = [4,2,7,9,15,3,0];
        let len = arr.len();
        partition(&mut arr, 0, len - 1);
        assert_eq!(arr, [0,2,7,9,15,3,4]);
    }
}