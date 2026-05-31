pub fn insertion_sort(x: &mut [i32]) {
    for i in 1..x.len() {
        let mut j = i;
        while j > 0 && x[j - 1] > x[j] {
            x.swap(j - 1, j);
            j -= 1;
        }
    }
}
