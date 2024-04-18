pub fn quick_sort<T: PartialOrd>(array: &mut [T]) {
    if array.len() <= 1 {
        return;
    }
    let pivot_index = partition(array);
    quick_sort(&mut array[0..pivot_index]);
    quick_sort(&mut array[pivot_index + 1..]);
}

fn partition<T: PartialOrd>(array: &mut [T]) -> usize {
    let pivot_index = array.len() / 2;
    array.swap(pivot_index, array.len() - 1);
    let mut i = 0;
    for j in 0..array.len() - 1 {
        if array[j] <= array[array.len() - 1] {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, array.len() - 1);
    i
}
