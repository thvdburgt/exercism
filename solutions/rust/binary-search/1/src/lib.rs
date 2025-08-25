/// Performs a binary search on a sorted slice.
/// Returns the index of `key` if found, or `None` otherwise.
pub fn find<T, U>(array: U, key: T) -> Option<usize>
where
    T: Ord,
    U: AsRef<[T]>,
{
    let slice = array.as_ref();
    let mut base = 0;
    let mut size = slice.len();

    while size > 1 {
        let half = size / 2;
        let mid = base + half;

        base = if slice[mid] > key { base } else { mid };
        size -= half;
    }

    slice
        .get(base)
        .and_then(|x| if *x == key { Some(base) } else { None })
}
