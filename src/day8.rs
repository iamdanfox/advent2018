#[cfg(test)]
mod test {
    use itertools::Itertools;

    #[test]
    fn parse_sample() {
        let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

        let vec = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        dbg!(&vec);
        dbg!(parse_entire_input(&vec));
    }

    fn parse_entire_input(input: &[usize]) -> Vec<LicenseNode> {
        let (vec, remainder) = parse_n_nodes(1, input);
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
        children: Vec<LicenseNode>,
        metadata: Vec<usize>,
    }
}
