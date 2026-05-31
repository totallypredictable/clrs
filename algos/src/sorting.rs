pub fn insertion_sort(x: &mut [i32]) -> &mut [i32] {
    for i in 1..x.len() {
        let key = x[i];
        let mut j = i - 1;
        while (j > 0) && (x[j] > key) {
            x[j + 1] = x[j];
            j -= 1;
        }
        x[j + 1] = key;
    }
    x
}
