pub fn selection_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 0..array.len() {
        let mut min_index = i;
        for j in i + 1..array.len() {
            if array[j] < array[min_index] {
                min_index = j;
            }
        }
        array.swap(i, min_index);
    }
}
