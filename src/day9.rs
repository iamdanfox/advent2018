#[cfg(test)]
mod test {
    use core::fmt::Write;
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::fmt::Error;
    use std::fmt::Formatter;

    #[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
    struct PlayerId(usize);

    // concept of a turn
    #[derive(Debug, PartialEq, Copy, Clone)]
    struct Marble(usize);

    struct Game {
        num_players: usize,
        num_marbles: usize,
        scores: HashMap<PlayerId, usize>,
        circle: Vec<Marble>,
        circle_current: usize,
        next_player: PlayerId,
        next_marble: Marble,
    }

    impl Game {
        // returns true iff game should continue (i.e. there are marbles remaining)
        fn next(&mut self) -> bool {
            let marble = self.next_marble;
            let index_to_insert = self.index_to_insert();

            self.circle.insert(index_to_insert, marble);
            self.circle_current = index_to_insert;

            if self.next_marble.0 + 1 == self.num_marbles {
                return false;
            }

            self.next_marble = Marble(self.next_marble.0 + 1);
            self.next_player = PlayerId(self.next_player.0 % self.num_players + 1);

            true
        }

        fn index_to_insert(&self) -> usize {
            if self.circle.len() == 1 {
                return 1;
            }

            let next = (self.circle_current + 2) % self.circle.len();
            if next == 0 {
                // prefer adding to the end of the vec rather than shifting everything along
                return self.circle.len();
            }

            next
        }
    }

    impl Debug for Game {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            f.write_char('[')?;
            self.next_player.0.fmt(f)?;
            f.write_char(']')?;

            for (index, marble) in self.circle.iter().enumerate() {
                if index == self.circle_current {
                    f.write_char('(')?;
                    marble.0.fmt(f)?;
                    f.write_char(')')?;
                } else {
                    f.write_char(' ')?;
                    marble.0.fmt(f)?;
                    f.write_char(' ')?;
                }
            }

            Ok(())
        }
    }

    #[test]
    fn sample_data() {
        let mut game = Game {
            num_players: 9,
            num_marbles: 25,
            scores: HashMap::new(),
            circle: vec![Marble(0)],
            circle_current: 0,
            next_player: PlayerId(1),
            next_marble: Marble(1),
        };

        while game.next() {
            dbg!(&game);
        }
    }
}
