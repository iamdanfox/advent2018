#[cfg(test)]
mod test {
    use core::fmt::Write;
    use itertools::Itertools;
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
        max_marble: usize,
        scores: HashMap<PlayerId, usize>,
        circle: Vec<Marble>,
        circle_current: usize,
        next_player: PlayerId,
        next_marble: Marble,
    }

    impl Game {
        fn new(num_players: usize, max_marble: usize) -> Game {
            Game {
                num_players,
                max_marble,
                scores: HashMap::new(),
                circle: vec![Marble(0)],
                circle_current: 0,
                next_player: PlayerId(1),
                next_marble: Marble(1),
            }
        }

        fn winner(&self) -> (PlayerId, usize) {
            let (player_id, score) = self
                .scores
                .iter()
                .max_by_key(|&(_, &score)| score)
                .expect("Must have at least one player");
            (*player_id, *score)
        }

        // returns true iff game should continue (i.e. there are marbles remaining)
        fn next(&mut self) -> bool {
            let marble = self.next_marble;

            if marble.0 % 23 == 0 {
                // the current player keeps the marble they would have placed, adding it to their score
                *self.scores.entry(self.next_player).or_default() += marble.0;

                // the marble 7 marbles counter-clockwise from the current marble is removed from the circle
                let index = self.index_backwards_7();
                let removed_marble = self.circle.remove(index);

                // and also added to the current player's score
                *self.scores.entry(self.next_player).or_default() += removed_marble.0;

                // The marble located immediately clockwise of the marble that was removed becomes the new current marble
                self.circle_current = index;

                return self.proceed_turn();
            }

            let index = self.index_forwards_2();
            self.circle.insert(index, marble);
            self.circle_current = index;

            self.proceed_turn()
        }

        fn proceed_turn(&mut self) -> bool {
            if self.next_marble.0 > self.max_marble {
                return false; // game ends
            }

            self.next_marble = Marble(self.next_marble.0 + 1);
            self.next_player = PlayerId(self.next_player.0 % self.num_players + 1);
            true
        }

        fn index_backwards_7(&self) -> usize {
            (self.circle_current + self.circle.len() - 7) % self.circle.len()
        }

        fn index_forwards_2(&self) -> usize {
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
    fn sample_data_1() {
        let mut game = Game::new(9, 25);
        while game.next() {}
        assert_eq!(game.winner(), (PlayerId(5), 32));
    }

    #[test]
    fn sample_data_2() {
        let mut game = Game::new(10, 1618);
        while game.next() {}
        assert_eq!(game.winner().1, 8317);
    }

    #[test]
    fn sample_data_3() {
        let mut game = Game::new(13, 7999);
        while game.next() {}
        assert_eq!(game.winner().1, 146373);
    }

    #[test]
    fn sample_data_4() {
        let mut game = Game::new(17, 1104);
        while game.next() {}
        assert_eq!(game.winner().1, 2764);
    }

    #[test]
    fn sample_data_5() {
        let mut game = Game::new(21, 6111);
        while game.next() {}
        assert_eq!(game.winner().1, 54718);
    }

    #[test]
    fn sample_data_6() {
        let mut game = Game::new(30, 5807);
        while game.next() {}
        assert_eq!(game.winner().1, 37305);
    }

    #[test]
    fn part1() {
        let mut game = Game::new(458, 72019);
        while game.next() {}
        assert_eq!(game.winner().1, 404502);
    }

    #[ignore]
    #[test]
    fn part2() {
        let mut game = Game::new(458, 72019 * 100);
        while game.next() {}
        assert_eq!(game.winner().1, 99999); // TODO(dfox) need to use something more efficient here
    }
}
