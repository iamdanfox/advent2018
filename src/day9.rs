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

    #[derive(Clone)]
    struct Game<T, C>
    where
        T: Circle<C>,
        T: Clone,
        C: Clone,
    {
        num_players: usize,
        max_marble: usize,
        scores: HashMap<PlayerId, usize>,
        circle: T,
        circle_current: C,
        next_player: PlayerId,
        next_marble: Marble,
    }

    trait Circle<Crsr> {
        fn len(&self) -> usize;
        fn get(&self, cursor: &Crsr) -> Option<&Marble>;
        fn seek_forward(&self, cursor: &Crsr, steps: usize) -> Crsr;
        fn seek_back(&self, cursor: &Crsr, steps: usize) -> Crsr;
        fn insert(&mut self, cursor: &Crsr, element: Marble);
        fn remove(&mut self, cursor: &Crsr) -> Marble;
        fn cursor_to_index(&self, cursor: &Crsr) -> usize;
        fn into_iter(self) -> Box<Iterator<Item = Marble>>;
    }

    fn new_game_segmented(num_players: usize, max_marble: usize) -> Game<SegmentedVec, Cursor> {
        let mut circle = SegmentedVec::default();
        let initial_cursor = Cursor::default();
        circle.insert(&initial_cursor, Marble(0));

        Game {
            num_players,
            max_marble,
            scores: HashMap::new(),
            circle,
            circle_current: initial_cursor,
            next_player: PlayerId(1),
            next_marble: Marble(1),
        }
    }

    fn new_game_flat(num_players: usize, max_marble: usize) -> Game<FlatVec, usize> {
        let mut circle = FlatVec::default();
        circle.insert(&0, Marble(0));

        Game {
            num_players,
            max_marble,
            scores: HashMap::new(),
            circle,
            circle_current: 0,
            next_player: PlayerId(1),
            next_marble: Marble(1),
        }
    }

    impl<T, C> Game<T, C>
    where
        T: Circle<C>,
        T: Clone,
        C: Clone,
    {

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
                let removed_marble = self.circle.remove(&index);

                // and also added to the current player's score
                *self.scores.entry(self.next_player).or_default() += removed_marble.0;

                // The marble located immediately clockwise of the marble that was removed becomes the new current marble
                self.circle_current = index;

                return self.proceed_turn();
            }

            let index = self.index_forwards_2();
            self.circle.insert(&index, marble);
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

        fn index_backwards_7(&self) -> C {
            self.circle.seek_back(&self.circle_current, 7)
        }

        fn index_forwards_2(&self) -> C {
            self.circle.seek_forward(&self.circle_current, 2)
        }
    }

    impl<T, C> Debug for Game<T, C>
    where
        T: Circle<C>,
        T: Clone,
        C: Clone,
    {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            f.write_char('[')?;
            self.next_player.0.fmt(f)?;
            f.write_char(']')?;

            let current_index = self.circle.cursor_to_index(&self.circle_current);

            for (index, marble) in self.circle.clone().into_iter().enumerate() {
                if index == current_index {
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

    #[derive(Default, Clone)]
    struct SegmentedVec {
        segments: Vec<Vec<Marble>>,
    }

    /// Points to a slot in the SegmentedVec, but there might not be an element there
    #[derive(Debug, PartialEq, Default, Clone)]
    struct Cursor {
        segment: usize,
        offset: usize,
    }

    impl Circle<Cursor> for SegmentedVec {
        fn len(&self) -> usize {
            self.segments.iter().map(|segment| segment.len()).sum()
        }

        fn get(&self, cursor: &Cursor) -> Option<&Marble> {
            let segment = self.segments.get(cursor.segment)?;
            segment.get(cursor.offset)
        }

        /// wraps around as necessary
        fn seek_forward(&self, cursor: &Cursor, steps: usize) -> Cursor {
            let current_segment = &self.segments[cursor.segment];

            // simplest case first - the desired offset is still within the current segment
            if cursor.offset + steps < current_segment.len() {
                return Cursor {
                    segment: cursor.segment,
                    offset: cursor.offset + steps,
                };
            }

            // otherwise, we need to move to a new segment
            let next_segment = (cursor.segment + 1) % self.segments.len();
            let remaining_steps = steps - (current_segment.len() - cursor.offset);
            let intermediate_cursor = Cursor {
                segment: next_segment,
                offset: 0,
            };

            self.seek_forward(&intermediate_cursor, remaining_steps)
        }

        fn seek_back(&self, cursor: &Cursor, steps: usize) -> Cursor {
            // simplest case first - the desired offset is still within the current segment
            if (cursor.offset as isize) - (steps as isize) >= 0 {
                return Cursor {
                    segment: cursor.segment,
                    offset: cursor.offset - steps,
                };
            }

            let next_segment = (cursor.segment as isize + self.segments.len() as isize - 1)
                as usize
                % self.segments.len();
            let remaining_steps = steps - cursor.offset;
            let intermediate_cursor = Cursor {
                segment: next_segment,
                offset: self.segments[next_segment].len(),
            };

            self.seek_back(&intermediate_cursor, remaining_steps)
        }

        fn insert(&mut self, cursor: &Cursor, element: Marble) {
            match self.segments.get_mut(cursor.segment) {
                Some(ref mut vec) if cursor.offset < vec.len() => {
                    vec.insert(cursor.offset, element);
                }
                _ => {
                    self.segments.push(vec![element]);
                }
            }
        }

        fn remove(&mut self, cursor: &Cursor) -> Marble {
            let segment = self.segments.get_mut(cursor.segment).unwrap();
            segment.remove(cursor.offset)
        }

        fn cursor_to_index(&self, cursor: &Cursor) -> usize {
            self.segments
                .iter()
                .take(cursor.segment)
                .map(|vec| vec.len())
                .sum::<usize>()
                + cursor.offset
        }

        fn into_iter(self) -> Box<Iterator<Item = Marble>> {
            struct SegmentedVecIter {
                vec: SegmentedVec,
                cursor: Cursor,
            }

            impl Iterator for SegmentedVecIter {
                type Item = Marble;

                fn next(&mut self) -> Option<Self::Item> {
                    let item = self.vec.get(&self.cursor).map(|&m| m);
                    self.cursor = self.vec.seek_forward(&self.cursor, 1);

                    item
                }
            }

            let len = self.len();
            let iter = SegmentedVecIter {
                vec: self,
                cursor: Cursor::default(),
            };
            Box::new(iter.take(len))
        }
    }

    #[derive(Default, Clone)]
    struct FlatVec {
        marbles: Vec<Marble>
    }

    impl Circle<usize> for FlatVec {
        fn len(&self) -> usize {
            self.marbles.len()
        }

        fn get(&self, &cursor: &usize) -> Option<&Marble> {
            self.marbles.get(cursor)
        }

        fn seek_forward(&self, &cursor: &usize, steps: usize) -> usize {
            (cursor + steps) % self.marbles.len()
        }

        fn seek_back(&self, &cursor: &usize, steps: usize) -> usize {
            ((cursor as isize) - (steps as isize) + (self.marbles.len() as isize)) as usize % self.marbles.len()
        }

        fn insert(&mut self, &cursor: &usize, element: Marble) {
            self.marbles.insert(cursor, element)
        }

        fn remove(&mut self, &cursor: &usize) -> Marble {
            self.marbles.remove(cursor)
        }

        fn cursor_to_index(&self, cursor: &usize) -> usize {
            *cursor
        }

        fn into_iter(self) -> Box<Iterator<Item=Marble>> {
            Box::new(self.marbles.into_iter())
        }
    }

    #[test]
    fn cursor_seek_wraps_nicely() {
        let s = SegmentedVec {
            segments: vec![
                vec![Marble(0), Marble(1), Marble(2)],
                vec![Marble(3), Marble(4)],
            ],
        };

        assert_eq!(
            s.seek_forward(
                &Cursor {
                    segment: 0,
                    offset: 0,
                },
                0,
            ),
            Cursor {
                segment: 0,
                offset: 0,
            }
        );
        assert_eq!(
            s.seek_forward(
                &Cursor {
                    segment: 0,
                    offset: 0,
                },
                1,
            ),
            Cursor {
                segment: 0,
                offset: 1,
            }
        );
        assert_eq!(
            s.seek_forward(
                &Cursor {
                    segment: 0,
                    offset: 0,
                },
                2,
            ),
            Cursor {
                segment: 0,
                offset: 2,
            }
        );
        assert_eq!(
            s.seek_forward(
                &Cursor {
                    segment: 0,
                    offset: 0,
                },
                3,
            ),
            Cursor {
                segment: 1,
                offset: 0,
            }
        );
        assert_eq!(
            s.seek_forward(
                &Cursor {
                    segment: 0,
                    offset: 0,
                },
                5,
            ),
            Cursor {
                segment: 0,
                offset: 0,
            }
        );

        assert_eq!(
            s.seek_back(
                &Cursor {
                    segment: 0,
                    offset: 2,
                },
                3,
            ),
            Cursor {
                segment: 1,
                offset: 1,
            }
        );
    }

    #[test]
    fn segmented_vec_iter_behaves() {
        let s = SegmentedVec {
            segments: vec![
                vec![Marble(0), Marble(1), Marble(2)],
                vec![Marble(3), Marble(4)],
            ],
        };

        assert_eq!(
            s.into_iter().collect_vec(),
            vec![Marble(0), Marble(1), Marble(2), Marble(3), Marble(4)]
        );
    }

    #[test]
    fn sample_data_1() {
        let mut game = new_game_segmented(9, 25);
        while game.next() {
            dbg!(&game);
        }
        assert_eq!(game.winner(), (PlayerId(5), 32));
    }

    #[test]
    fn comparative() {
        let marbles= 45;
        let mut segmented = new_game_segmented(9, marbles);
        let mut flat = new_game_flat(9, marbles);

        let rounds = 20;
        for i in 0..rounds {
            segmented.next();
            dbg!(&segmented);
        }
        for i in 0..rounds {
            flat.next();
            dbg!(&flat);
        }

        while segmented.next() {}
        while flat.next() {}

        assert_eq!(segmented.winner().1, flat.winner().1);
    }

    #[test]
    fn sample_data_2() {
        let mut game = new_game_segmented(10, 1618);
        while game.next() {}
        assert_eq!(game.winner().1, 8317);
    }

    #[test]
    fn sample_data_3() {
        let mut game = new_game_segmented(13, 7999);
        while game.next() {}
        assert_eq!(game.winner().1, 146373);
    }

    #[test]
    fn sample_data_4() {
        let mut game = new_game_segmented(17, 1104);
        while game.next() {}
        assert_eq!(game.winner().1, 2764);
    }

    #[test]
    fn sample_data_5() {
        let mut game = new_game_segmented(21, 6111);
        while game.next() {}
        assert_eq!(game.winner().1, 54718);
    }

    #[test]
    fn sample_data_6() {
        let mut game = new_game_segmented(30, 5807);
        while game.next() {}
        assert_eq!(game.winner().1, 37305);
    }

    #[test]
    fn part1() {
        let mut game = new_game_segmented(458, 72019);
        while game.next() {}
        assert_eq!(game.winner().1, 404502);
    }

    #[ignore]
    #[test]
    fn part2() {
        let mut game = new_game_segmented(458, 72019 * 100);
        while game.next() {}
        assert_eq!(game.winner().1, 99999); // TODO(dfox) need to use something more efficient here
    }
}
