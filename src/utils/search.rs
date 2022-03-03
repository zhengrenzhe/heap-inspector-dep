use std::collections::HashMap;

pub fn count_same_string(data: &[(&str, usize)], more_than_times: usize) -> Vec<usize> {
    // string value, [string id]
    let mut string_result: HashMap<&str, Vec<&usize>> = HashMap::new();

    data.iter().for_each(|(string_value, string_id)| {
        if string_result.contains_key(*string_value) {
            let s = string_result.get_mut(*string_value).unwrap();
            s.push(string_id);
        } else {
            string_result.insert(string_value, vec![string_id]);
        }
    });

    let mut result: Vec<usize> = vec![];

    string_result.values().for_each(|ids| {
        if ids.len() > more_than_times {
            for id in ids {
                result.push(**id);
            }
        }
    });

    string_result.clear();

    result
}

#[cfg(test)]
mod tests {
    use crate::utils::search::count_same_string;

    #[test]
    fn test_count_same_string() {
        let data = vec![
            // a
            ("a", 1),
            ("a", 2),
            ("a", 3),
            ("a", 4),
            // b
            ("b", 5),
            ("b", 6),
            ("b", 7),
            // c
            ("c", 8),
            ("c", 9),
            // d
            ("d", 10),
        ];

        let mut a = count_same_string(&data, 1);
        a.sort_unstable();
        assert_eq!(a, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut b = count_same_string(&data, 2);
        b.sort_unstable();
        assert_eq!(b, vec![1, 2, 3, 4, 5, 6, 7]);

        let mut c = count_same_string(&data, 3);
        c.sort_unstable();
        assert_eq!(c, vec![1, 2, 3, 4]);

        assert_eq!(count_same_string(&data, 4), vec![] as Vec<usize>);
    }

    #[test]
    fn test2() {
        let data = vec![String::from("bar")];
        let s = "foo a bar";

        assert!(data.iter().any(|ss| s.contains(ss)));
        assert!(!(vec![] as Vec<String>).iter().any(|ss| ss.contains(s)),);
    }
}
