// Takes a Vec<T> and returns the same if nonempty, or vec![None] otherwise
pub fn ensure_nonempty<T>(inp: Vec<T>) -> Vec<Option<T>> {
    if inp.is_empty() {
        vec![None]
    } else {
        inp.into_iter().map(move |v| Some(v)).collect()
    }
}
