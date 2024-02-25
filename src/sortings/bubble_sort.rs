
pub fn bubble_sort_silly(mut array: Vec<i32>) -> Vec<i32> {
    for _ in 0..array.len() {
        for i in 0..array.len() - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
            }
        }
    }

    array
}

#[test]
fn should_bubble_sort_silly() {
    let arr: Vec<i32> = vec![
        5, 2, 3, 4, 1, 5, 2, 3, 4, 1, 5, 2, 3, 422, 1, 5, 2, 3, 4, 1, 5, 2, 3, 4, 1,
    ];
    assert_eq!(bubble_sort_silly(arr.clone()), [1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 422]);
}

pub fn bubble_sort(mut array: Vec<i32>) -> Vec<i32> {
    let n = array.len();
    let mut swapped;

    for i in 0..n {
        swapped = false;

        for j in 0..n - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    array
}


#[test]
fn should_bubble_sort() {
    let arr: Vec<i32> = vec![
        5, 2, 3, 4, 1, 5, 2, 3, 4, 1, 5, 2, 3, 422, 1, 5, 2, 3, 4, 1, 5, 2, 3, 4, 1,
    ];

    assert_eq!(bubble_sort(arr.clone()), [1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 422]);
}