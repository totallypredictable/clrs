mod sorting;
use sorting::insertion_sort;

fn main() {
    let mut my_list = [1, 3, 2, 6, 5, 4];
    println!("{:?}", insertion_sort(&mut my_list));
}
