use crate::sort::Sorter;
use std::fmt::Debug;

struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T: Ord + Debug>(slice: &mut [T]) {
        // 1. build max heap
        build(slice);
        // 2. remove max/root element put at the end and repair the heap
        swap_and_repair(slice);
    }
}

fn swap_and_repair<T>(slice: &mut [T])
where
    T: Ord + Debug,
{
    if slice.len() < 2 {
        return;
    }
    slice.swap(0, slice.len() - 1);
    let [heap @ .., _] = slice else {
        panic!("should have at least two elements")
    };
    // repair
    bubble_down(heap, 0);
    swap_and_repair(heap);
}

fn bubble_down<T: Ord + Debug>(slice: &mut [T], p: usize) {
    // take bigger of the child and swap with the parent if it is bigger then parent

    let ch1 = 2 * p + 1;
    if ch1 >= slice.len() {
        return;
    }

    let ch2 = ch1 + 1;
    let b = if ch2 >= slice.len() || slice[ch1] > slice[ch2] {
        ch1
    } else {
        ch2
    };

    if slice[b] > slice[p] {
        slice.swap(b, p);
        bubble_down(slice, b); // bubble down original element that was on p and now is on b index
    }
}

fn build<T: Ord + Debug>(slice: &mut [T]) {
    // array <-> binary heap mapping
    // ch1 = 2*p + 1
    // ch2 = 2*p + 2

    // from the middle of array to the beginning, which means upper half of the tree
    let mid = slice.len() / 2;
    for i in (0..=mid).rev() {
        bubble_down(slice, i);
    }
}

#[test]
fn build_works() {
    let mut a = [1, 2, 4, 5, 8, 3, 9, 8, 10];
    build(&mut a);
    assert_eq!(a, [10, 8, 9, 8, 1, 3, 4, 2, 5]);
}

#[test]
fn test_selection_sort() {
    use crate::sort::tests::test_sorting;
    test_sorting::<SelectionSort>();
}
