mod sorting;
use sorting::insertion_sort;

fn main() {
    let mut my_list = [6, 3, 2, 1, 5, 4];
    println!("{:?}", insertion_sort(&mut my_list, true));
}
