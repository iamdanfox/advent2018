#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::fs;

    static DELETED: u8 = 48; // 0

    #[test]
    fn part1() {
        let mut bytes: Vec<u8> = fs::read("day5.txt").unwrap();
        println!("Loaded {} bytes from file", &bytes.len());

        let all_known_bytes: HashSet<u8> = bytes.iter().cloned().collect();
        assert_eq!(all_known_bytes.contains(&DELETED), false);

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
            println!("new_vec is {} bytes long", new_vec.len());
            bytes = new_vec;
        }

        assert_eq!(std::str::from_utf8(bytes.as_ref()).unwrap().trim().len(), 11194);
    }

    #[inline]
    fn should_collapse(left: u8, right: u8) -> bool {
        left.eq_ignore_ascii_case(&right) && (char::from(left).is_lowercase() ^ char::from(right).is_lowercase())
    }
}
