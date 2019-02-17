#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::fs;

    static DELETED: u8 = 48; // 0

    #[test]
    fn part1() {
        let mut bytes: Vec<u8> = fs::read("day5.txt").unwrap();
        let squished: Vec<u8> = recursively_collapse(&mut bytes);

        let x = std::str::from_utf8(&squished).unwrap();
        assert_eq!(x.trim().len(), 11194);
    }

    #[test]
    fn part2() {
        let string = fs::read_to_string("day5.txt").unwrap();
        let mut best = string.len();

        for letter in b'A'..b'Z' {
            let cleansed = string
                .trim()
                .replace(letter as char, "")
                .replace((letter as char).to_ascii_lowercase(), "");
            let vec = recursively_collapse(cleansed.as_bytes());
            let candidate = dbg!(vec.len());
            if candidate < best {
                best = candidate;
            }
        }

        assert_eq!(best, 4178);
    }

    fn recursively_collapse(entry: &[u8]) -> Vec<u8> {
        println!("Starting with {} bytes", &entry.len());

        let mut bytes: Vec<u8> = Vec::from(entry);
        loop {
            let mut changes = false;
            for i in 0..bytes.len() - 1 {
                if should_collapse(bytes[i], bytes[i + 1]) {
                    bytes[i] = DELETED;
                    bytes[i + 1] = DELETED;
                    changes = true
                }
            }

            if !changes {
                break;
            }

            let mut new_vec = Vec::new();
            for i in 0..bytes.len() {
                if bytes[i] != DELETED {
                    new_vec.push(bytes[i]);
                }
            }
            bytes = new_vec;
        }
        bytes
    }

    fn alphabet() -> HashSet<char> {
        let string = fs::read_to_string("day5.txt").unwrap().to_uppercase();
        string.chars().collect()
    }

    #[test]
    fn check_alphabet_of_polymers() {
        let all_known_bytes = alphabet();
        assert_eq!(all_known_bytes.len(), 27); // there's a newline on the end!
    }

    #[inline]
    fn should_collapse(left: u8, right: u8) -> bool {
        left.eq_ignore_ascii_case(&right)
            && (char::from(left).is_lowercase() ^ char::from(right).is_lowercase())
    }
}
