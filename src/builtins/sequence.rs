/// Returns the number of items in the array parameter.
/// If the array parameter is not an array, but rather a
/// value of another JSON type, then the parameter is
/// treated as a singleton array containing that value,
/// and this function returns 1.
fn count<T>(arr: Vec<T>) -> usize {
    arr.len()
}
