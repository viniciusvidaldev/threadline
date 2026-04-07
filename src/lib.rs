fn lcs<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<&'a str> {
    let old_len = old.len();
    let new_len = new.len();
    let mut matrix = vec![vec![0usize; new.len() + 1]; old.len() + 1];

    for i in 1..=old_len {
        for j in 1..=new_len {
            let old_line = old[i - 1];
            let new_line = new[j - 1];

            matrix[i][j] = if old_line == new_line {
                matrix[i - 1][j - 1] + 1
            } else {
                matrix[i - 1][j].max(matrix[i][j - 1])
            };
        }
    }

    let mut result = Vec::new();
    let mut i = old_len;
    let mut j = new_len;

    while i > 0 && j > 0 {
        if old[i - 1] == new[j - 1] {
            result.push(old[i - 1]);
            i -= 1;
            j -= 1;
        } else if matrix[i - 1][j] > matrix[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    result.reverse();
    result
}

pub enum DiffLine<'a> {
    Added(&'a str),
    Removed(&'a str),
    Unchanged(&'a str),
}

pub fn diff<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<DiffLine<'a>> {
    let common = lcs(old, new);
    let mut result = Vec::new();

    let mut old_idx = 0;
    let mut new_idx = 0;
    let mut common_idx = 0;

    while common_idx < common.len() {
        let anchor = common[common_idx];

        while old_idx < old.len() && old[old_idx] != anchor {
            result.push(DiffLine::Removed(old[old_idx]));
            old_idx += 1;
        }
        while new_idx < new.len() && new[new_idx] != anchor {
            result.push(DiffLine::Added(new[new_idx]));
            new_idx += 1;
        }

        result.push(DiffLine::Unchanged(anchor));
        old_idx += 1;
        new_idx += 1;
        common_idx += 1;
    }

    while old_idx < old.len() {
        result.push(DiffLine::Removed(old[old_idx]));
        old_idx += 1;
    }
    while new_idx < new.len() {
        result.push(DiffLine::Added(new[new_idx]));
        new_idx += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_lines() {
        let old = vec!["A", "B", "C", "B"];
        let new = vec!["B", "C", "B"];
        assert_eq!(lcs(&old, &new), vec!["B", "C", "B"]);
    }

    #[test]
    fn test_no_common_lines() {
        let old = vec!["A", "B"];
        let new = vec!["C", "D"];
        assert_eq!(lcs(&old, &new), Vec::<&str>::new());
    }

    #[test]
    fn test_identical_files() {
        let old = vec!["A", "B", "C"];
        let new = vec!["A", "B", "C"];
        assert_eq!(lcs(&old, &new), vec!["A", "B", "C"]);
    }

    #[test]
    fn test_one_empty() {
        let old = vec!["A", "B", "C"];
        let new = vec![];
        assert_eq!(lcs(&old, &new), Vec::<&str>::new());
    }

    #[test]
    fn test_single_common_line() {
        let old = vec!["A", "B", "C"];
        let new = vec!["X", "B", "Y"];
        assert_eq!(lcs(&old, &new), vec!["B"]);
    }

    #[test]
    fn test_subsequence_order_matters() {
        let old = vec!["B", "A", "B"];
        let new = vec!["A", "B", "A"];
        assert_eq!(lcs(&old, &new).len(), 2);
    }

    #[test]
    fn test_diff() {
        let old = vec!["A", "B", "C"];
        let new = vec!["A", "X", "C"];

        let result = diff(&old, &new);

        assert!(matches!(result[0], DiffLine::Unchanged("A")));
        assert!(matches!(result[1], DiffLine::Removed("B")));
        assert!(matches!(result[2], DiffLine::Added("X")));
        assert!(matches!(result[3], DiffLine::Unchanged("C")));
    }
}
