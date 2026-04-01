// test data to ensure enum roundtripping is correct
// each of these functions creates a file that has every enum value,
// and returns a list with the expected order.

use strum::VariantArray;

/// support function for enums - get the i-th element of T, or None if there are not that many elements.
pub fn get_or_none<T: Copy + VariantArray>(i: usize) -> Option<T> {
    if i >= T::VARIANTS.len() {
        None
    } else {
        Some(T::VARIANTS[i])
    }
}
