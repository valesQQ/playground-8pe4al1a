#[cfg(test)]
extern crate rust_template;

use rust_template::tatooine::find_tatooine;

#[test]
pub fn test_tatooine() {
    let systems = [
        1, 1, 2, 9, 6, 9, 9, 8, 1, 8, 8, 1, 8, 2, 9, 6, 5, 4, 2, 8, 4, 4, 8, 5, 5, 3, 6, 4, 6, 7,
        4, 7, 4, 6, 3, 1, 5, 7, 5, 5, 8, 2, 6, 7, 7, 9, 7, 7, 1, 4, 4, 4, 9, 6, 5, 7, 8, 6, 8, 8,
        8, 9, 5, 4, 1, 3, 3, 8, 5, 1, 5, 3, 5, 6, 7, 8, 8, 8, 4, 8, 5, 5, 4, 6, 4, 4, 8, 5, 9, 7,
        9, 7, 2, 7, 5, 8, 3, 9, 4, 9,
    ];
    let expected = 5;

    assert_eq!(expected, find_tatooine(&systems));
}
