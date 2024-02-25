pub fn merge_sort(mut array: Vec<i32>) -> Vec<i32> {
    let len = array.len();

    if len <= 1 {
        return array;
    }

    let mid = len / 2;
    let mut left: Vec<i32> = array.drain(..mid).collect();
    let mut right = array;

    left = merge_sort(left);
    right = merge_sort(right);

    let mut merged = Vec::with_capacity(left.len() + right.len());

    while !left.is_empty() && !right.is_empty() {
        if left[0] <= right[0] {
            merged.push(left.remove(0));
        } else {
            merged.push(right.remove(0));
        }
    }

    merged.extend(left);
    merged.extend(right);

    merged
}

#[test]
fn should_merge_sort() {
    let arr: Vec<i32> = vec![
        5, 2, 3, 4, 1, 5, 2, 3, 4, 1, 5, 2, 3, 422, 1, 5, 2, 3, 4, 1, 5, 2, 3, 4, 1,
    ];
    assert_eq!(merge_sort(arr.clone()), [1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 422]);
}
