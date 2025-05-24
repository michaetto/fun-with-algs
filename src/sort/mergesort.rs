use crate::sort::Sorter;
use std::ptr;

pub struct MergeSort;
impl Sorter for MergeSort {
    fn sort<T: Ord + Clone>(slice: &mut [T]) {
        // 1. recursively split in half (top-down in tree)
        // 2. then merge each one (bottom-up) by preserving order
        // - (so that created new nodes are sorted)
        let mut cloned = slice.to_vec();
        mergesort(&mut cloned, slice);
    }
}

fn merge<'a, T: Ord + Clone>(left: &'a [T], right: &'a [T], out: &mut [T]) {
    // merge left and right into out
    // by taking on by one from left and right
    // and chosing smaller, so that out is sorted
    assert_eq!(left.len() + right.len(), out.len());
    let mut left_iter = left.iter().peekable();
    let mut right_iter = right.iter().peekable();
    let mut out_iter = out.iter_mut();
    loop {
        match (left_iter.peek(), right_iter.peek()) {
            (None, None) => break,
            (None, Some(..)) => {
                *out_iter.next().expect("invalid out") = right_iter.next().unwrap().clone();
            }
            (Some(..), None) => {
                *out_iter.next().expect("invalid out") = left_iter.next().unwrap().clone();
            }
            (Some(&l), Some(&r)) => {
                // take smaller, so that
                if l <= r {
                    *out_iter.next().expect("invalid out") = left_iter.next().unwrap().clone();
                } else {
                    *out_iter.next().expect("invalid out") = right_iter.next().unwrap().clone();
                }
            }
        }
    }
}

fn mergesort<T: Ord + Clone>(slice_in: &mut [T], slice_out: &mut [T]) {
    if slice_in.len() <= 1 {
        return;
    }

    // split into binary tree
    let mid = slice_in.len() / 2;
    let (left_in, right_in) = slice_in.split_at_mut(mid);
    let (left_out, right_out) = slice_out.split_at_mut(mid);

    mergesort(left_in, left_out);
    mergesort(right_in, right_out);
    unsafe {
        if left_out.len() > 1 {
            ptr::copy_nonoverlapping(left_out.as_ptr(), left_in.as_mut_ptr(), left_out.len());
        }
        if right_out.len() > 1 {
            ptr::copy_nonoverlapping(right_out.as_ptr(), right_in.as_mut_ptr(), right_out.len());
        }
    }
    merge(left_in, right_in, slice_out);
}

#[test]
fn test_selection_sort() {
    use crate::sort::tests::test_sorting;
    test_sorting::<MergeSort>();
}
