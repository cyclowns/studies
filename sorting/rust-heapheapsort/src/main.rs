fn main() {
    let mut arr = vec![1, 3, 2, 9, 5, 4, 11, 6];
    let heap = into_maxheap(&mut arr);
    println!("{:?}", heap);
}

fn into_maxheap<T: Ord + Copy + Sized>(mut array: &mut [T]) -> &[T] {
    for idx in ((array.len() / 2) - 1)..0 {
        max_heapify(&mut array, idx);
    }
    array
}

fn max_heapify<T: Ord + Copy + Sized>(array: &mut [T], idx: usize) {
    let mut to_swap = idx.clone();
    while !is_leaf(array.len(), to_swap) {
        let mut max_child = left_leaf(to_swap); // we can change to right leaf if we are wrong
        if (max_child < array.len() - 1) && array[max_child] < array[max_child + 1] {
            max_child += 1;
        }

        if array[to_swap] >= array[max_child] {
            return; // no need to switch
        }

        array.swap(to_swap, max_child);
        to_swap = max_child;
    }
}

// check if it is a leaf
fn is_leaf(len: usize, idx: usize) -> bool {
    idx >= len / 2
}

// get left leaf idx of a parent node
fn left_leaf(idx: usize) -> usize {
    (2 * idx) + 1
}

// get right leaf idx of a parent node
fn right_leaf(idx: usize) -> usize {
    (2 * idx) + 2
} 

//fn heapsort<T: Ord>(array: &[T]) -> [T] {
//
//}