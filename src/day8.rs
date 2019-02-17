#[cfg(test)]
mod test {
    use itertools::Itertools;
    use std::fs;

    #[test]
    fn real_data() {
        let string = fs::read_to_string("day8.txt").unwrap();
        let node = &parse_input(&string)[0];

        assert_eq!(sum_metadata(node), 37905);
        assert_eq!(value_of_node(node), 33891);
    }

    #[test]
    fn sample_data() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        let node = &parse_input(input)[0];

        assert_eq!(sum_metadata(node), 138);
        assert_eq!(value_of_node(node), 66);
    }

    fn value_of_node(l: &LicenseNode) -> usize {
        if l.children.is_empty() {
            return sum_metadata(l);
        }

        let mut sum = 0;
        for &index in &l.metadata {
            if let Some(child_node) = l.children.get(index - 1) {
                sum += value_of_node(&child_node);
            }
        }

        sum
    }

    fn sum_metadata(l: &LicenseNode) -> usize {
        let own = l.metadata.iter().sum::<usize>();
        let children = l
            .children
            .iter()
            .map(|child| sum_metadata(child))
            .sum::<usize>();
        own + children
    }

    fn parse_input(input: &str) -> Vec<LicenseNode> {
        let vec = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();

        let (vec, remainder) = parse_n_nodes(1, &vec);
        assert!(remainder.is_empty(), "entire input should be consumed");
        assert_eq!(vec.len(), 1, "exactly one node should be returned");
        vec
    }

    // return remainder of input
    fn parse_n_nodes(expected_children: usize, input: &[usize]) -> (Vec<LicenseNode>, &[usize]) {
        if expected_children == 0 {
            return (Vec::new(), input);
        }

        let mut vec = Vec::new();
        let mut remaining = input;

        for _ in 0..expected_children {
            let num_children = remaining[0];
            let num_metadata = remaining[1];
            let (parsed, unprocessed) = parse_n_nodes(num_children, &remaining[2..]);

            vec.push(LicenseNode {
                children: parsed,
                metadata: Vec::from(&unprocessed[0..num_metadata]),
            });
            remaining = &unprocessed[num_metadata..];
        }

        (vec, remaining)
    }

    #[derive(Debug)]
    struct LicenseNode {
        pub children: Vec<LicenseNode>,
        pub metadata: Vec<usize>,
    }
}
