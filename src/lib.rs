pub fn mergesort<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    if list.len() <= 1 {
        return vec![list[0].clone()];
    }

    // splitting function
    let split = |list: &[T]| {
        let mid = list.len() >> 1;
        (mergesort(&list[..mid]), mergesort(&list[mid..]))
    };

    let (a, b) = split(list);

    // merging function
    let mut i = 0;
    let mut j = 0;

    let mut result = Vec::new();
    loop {
        // if no elements remain in a, just fill up result with b
        if i >= a.len() {
            while j < b.len() {
                result.push(b[j].clone());
                j += 1;
            }
            break;
        }

        // if no elements remain in b, just fill up result with a
        if j >= b.len() {
            while i < a.len() {
                result.push(a[i].clone());
                i += 1;
            }
            break;
        }

        if a[i] > b[j] {
            // b_j is smaller and comes first
            result.push(b[j].clone());
            j += 1;
        } else {
            // a_i is smaller and comes first
            result.push(a[i].clone());
            i += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::mergesort;
    #[test]
    fn ordered_list() {
        let input = [1, 2, 3, 4, 5];
        let expected = [1, 2, 3, 4, 5];
        let result = mergesort(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn unordered_list() {
        let input = [10, 0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
        let expected = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = mergesort(&input);

        assert_eq!(result, expected);
    }
}
