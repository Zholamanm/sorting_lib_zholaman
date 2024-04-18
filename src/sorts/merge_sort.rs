pub fn merge_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    if array.len() > 1 {
        let mid = array.len() / 2;
        let mut left = array[..mid].to_vec();
        let mut right = array[mid..].to_vec();

        merge_sort(&mut left);
        merge_sort(&mut right);

        merge(array, &left, &right);
    }
}

fn merge<T: PartialOrd + Clone>(array: &mut [T], left: &[T], right: &[T]) {
    let (mut left_index, mut right_index, mut i) = (0, 0, 0);
    while left_index < left.len() && right_index < right.len() {
        if left[left_index] <= right[right_index] {
            array[i] = left[left_index].clone();
            left_index += 1;
        } else {
            array[i] = right[right_index].clone();
            right_index += 1;
        }
        i += 1;
    }

    while left_index < left.len() {
        array[i] = left[left_index].clone();
        left_index += 1;
        i += 1;
    }

    while right_index < right.len() {
        array[i] = right[right_index].clone();
        right_index += 1;
        i += 1;
    }
}
