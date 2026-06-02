#[allow(dead_code)]
pub fn insertion_sort(x: &mut [i32], reverse: bool) -> &mut [i32] {
    for i in 1..x.len() {
        let mut j = i;
        while j > 0
            && (if !reverse {
                x[j - 1] > x[j]
            } else {
                x[j - 1] < x[j]
            })
        {
            x.swap(j - 1, j);
            j -= 1;
        }
    }
    x
}

fn merge(arr: &mut [i32], p: usize, q: usize, r: usize) {
    let n_left = q - p + 1;
    let n_right = r - q;
    let left: Vec<i32> = Vec::from(&arr[p..p + n_left]);
    let right: Vec<i32> = Vec::from(&arr[p + n_left..p + n_left + n_right]);

    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    while i < n_left && j < n_right {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < n_left {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < n_right {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

pub fn merge_sort(arr: &mut [i32], p: usize, r: usize) {
    if p >= r {
        return;
    }
    let q = (p + r) / 2;
    merge_sort(arr, p, q);
    merge_sort(arr, q + 1, r);
    merge(arr, p, q, r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [2, 5, 1, 6, 4, 3, 8, 7];
        let r = arr.len() - 1;
        merge_sort(&mut arr, 0, r);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = [2, 5, 1, 6, 4, 3, 8, 7];
        insertion_sort(&mut arr, false);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_reverse_insertion_sort() {
        let mut arr = [2, 5, 1, 6, 4, 3, 8, 7];
        insertion_sort(&mut arr, true);
        assert_eq!(arr, [8, 7, 6, 5, 4, 3, 2, 1]);
    }
}
