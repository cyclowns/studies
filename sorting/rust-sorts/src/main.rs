#[cfg(test)]
mod tests;

fn main() {
    let example_vec = vec!['e', 'd', 'c', 'b', 'a'];
    println!("{:?}", selection_sort(&example_vec));
}

// selection sort
fn minimum_index<T: Ord + Copy>(array: &[T], start: usize) -> usize {
    let mut min_value: T = array[start];
    let mut min_index = start;

    for i in start..array.len() {
        if array[i] < min_value {
            min_value = array[i];
            min_index = i;
        }
    }

    min_index
}

fn selection_sort<T: Ord + Copy>(array: &[T]) -> Vec<T> {
    let mut sorted_vec = array.clone().to_vec();

    for i in 0..array.len() {
        let min_subarray_index = minimum_index(&sorted_vec, i);
        sorted_vec.swap(i, min_subarray_index);
    }

    sorted_vec
}
