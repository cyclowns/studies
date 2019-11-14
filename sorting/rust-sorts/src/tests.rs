use crate::main::*;
#[test]
fn selection_sort() {
    let mut sort = vec![0, 3, 2, 1];
    assert_eq!(selection_sort(&sort), &[0, 1, 2, 3]);
}