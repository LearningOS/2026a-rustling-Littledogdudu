/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.

    Solution: quicksort with a median-of-three pivot.
    Average O(n log n) time, O(log n) stack, in-place, and it only needs
    `PartialOrd` (not `Ord`), which is the tightest bound the tests allow.
*/

/// Quicksort driver. Recurses only on slices longer than one element.
fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    let (left, right) = arr.split_at_mut(pivot);
    quick_sort(left);
    quick_sort(&mut right[1..]); // skip the pivot itself
}

/// Lomuto partition. The pivot is moved to the end first.
fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let last = arr.len() - 1;
    let median = median_of_three(arr);
    arr.swap(median, last);

    let mut store = 0;
    for i in 0..last {
        if arr[i] < arr[last] {
            arr.swap(i, store);
            store += 1;
        }
    }
    arr.swap(store, last);
    store
}

/// Returns the index holding the median of the first / middle / last element,
/// so that already-sorted input does not hit the O(n^2) worst case.
fn median_of_three<T: PartialOrd>(arr: &mut [T]) -> usize {
    let (a, b, c) = (0, arr.len() / 2, arr.len() - 1);
    if arr[a] > arr[b] {
        arr.swap(a, b);
    }
    if arr[b] > arr[c] {
        arr.swap(b, c);
    }
    if arr[a] > arr[b] {
        arr.swap(a, b);
    }
    b
}

/// Bubble sort with an early-exit when a pass makes no swaps.
#[allow(dead_code)]
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let mut end = arr.len();
    while end > 1 {
        let mut sorted = true;
        for i in 1..end {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
        end -= 1;
    }
}

/// Insertion sort (shifts instead of swapping, so it is O(n + inversions)).
#[allow(dead_code)]
fn insert_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn sort<T: PartialOrd>(array: &mut [T]) {
    quick_sort(array);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    #[test]
    fn test_sort_empty_and_duplicates() {
        let mut empty: Vec<i32> = vec![];
        sort(&mut empty);
        assert!(empty.is_empty());

        let mut dup = vec![3, 1, 3, 2, 1, 3];
        sort(&mut dup);
        assert_eq!(dup, vec![1, 1, 2, 3, 3, 3]);
    }

    #[test]
    fn test_other_sorts() {
        let mut a = vec![5, 2, 9, 1, 5, 6];
        bubble_sort(&mut a);
        assert_eq!(a, vec![1, 2, 5, 5, 6, 9]);

        let mut b = vec![5, 2, 9, 1, 5, 6];
        insert_sort(&mut b);
        assert_eq!(b, vec![1, 2, 5, 5, 6, 9]);
    }
}
