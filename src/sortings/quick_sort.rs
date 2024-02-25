fn quick_sort(mut array: Vec<i32>) -> Vec<i32> {
    if array.len() <= 1 {
        return array;
    }

    let p = array.remove(0);
    let left = array.iter().filter(|&x| *x <= p).cloned().collect();
    let right = array.into_iter().filter(|&x| x > p).collect();

    let mut sorted = quick_sort(left);

    sorted.push(p);
    sorted.append(&mut quick_sort(right));

    sorted
}

#[test]
fn should_quick_sort() {
    let arr: Vec<i32> = vec![
        5, 2, 3, 4, 1, 5, 2, 3, 4, 1, 5, 2, 3, 422, 1, 5, 2, 3, 4, 1, 5, 2, 3, 4, 1,
    ];
    assert_eq!(quick_sort(arr.clone()), [1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 422]);
}
