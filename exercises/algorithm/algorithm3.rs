/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::fmt::Display;
use std::fmt::Debug;

/**
 * 冒泡排序
 */
fn bubble_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Display
{
    let mut length = arr.len();
    let mut is_sorted = true;

    for i in 0..length - 1 {
        is_sorted = true;
        for j in 1..length {
            if arr[j - 1] > arr[j] {
                is_sorted = false;
                arr.swap(j - 1, j);
            }
        }
        length -= 1;
        if is_sorted {
            break;
        }
    }
}

/**
 * 插入排序
 */
fn insert_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Display + Debug
{
    let length = arr.len();

    for i in 1..length {
        let mut insert_index = i;
        for j in (0..i).rev() {
            if arr[j] < arr[insert_index] {
                break;
            }
            arr.swap(insert_index, j);
            insert_index = j;
            println!("{:?}", arr);
        }
    }
}

/**
 * 快速排序
 */
fn partial_sort<T>(arr: &mut [T]) -> usize
where
    T: PartialOrd + Display + Debug
{
    let length = arr.len() - 1;
    let mut pivot_index = 0;
    for index in 0..length {
        if arr[index] < arr[length] {
            arr.swap(index, pivot_index);
            pivot_index += 1;
        }
    }
    arr.swap(pivot_index, length);

    pivot_index
}

fn quick_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Display + Debug
{
    let length = arr.len();
    if length <= 1 {
        return;
    }
    let key = partial_sort(arr);
    quick_sort(&mut arr[0..key]);
    quick_sort(&mut arr[key + 1..]);
}

fn sort<T>(array: &mut [T])
where
    T: PartialOrd + Display + Debug
{
	// bubble_sort(array);
    // insert_sort(array);
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
}
