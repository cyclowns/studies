#[cfg(test)]
mod tests;

fn main() {
    let example_vec = vec!['2', '1', '3', '5', '4'];
    println!("sel: {:?}", selection_sort(&example_vec));
    println!("ins: {:?}", insertion_sort(&example_vec));
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

// insertion sort
fn insertion_sort<T: Ord + Copy>(array: &[T]) -> Vec<T> {
    let mut sorted_vec = array.clone().to_vec();
    let len = sorted_vec.len();

    for i in 1..len {
        let mut j = i - 1;

        while sorted_vec[j + 1] < sorted_vec[j] {
            sorted_vec.swap(j + 1, j);
            if j != 0 { 
                j -= 1 
            } else {
                break;
            }
        }
    }
    sorted_vec
}
