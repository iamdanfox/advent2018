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
        circle: SegmentedVec,
        circle_current: Cursor,
        next_player: PlayerId,
        next_marble: Marble,
    }

    impl Game {
        fn new(num_players: usize, max_marble: usize) -> Game {
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

        fn index_backwards_7(&self) -> Cursor {
            self.circle.seek_back(&self.circle_current, 7)
        }

        fn index_forwards_2(&self) -> Cursor {
            self.circle.seek_forward(&self.circle_current, 2)

            //            if self.circle.len() == 1 {
            //                return 1;
            //            }
            //
            //            let next = (self.circle_current + 2) % self.circle.len();
            //            if next == 0 {
            //                // prefer adding to the end of the vec rather than shifting everything along
            //                return self.circle.len();
            //            }
            //
            //            next
        }
    }

    impl Debug for Game {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            f.write_char('[')?;
            self.next_player.0.fmt(f)?;
            f.write_char(']')?;

            let current_index = self.circle.cursor_to_index(&self.circle_current);

            for (index, marble) in self.circle.iter().enumerate() {
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

    #[derive(Default)]
    struct SegmentedVec {
        marbles: Vec<Marble>,
    }

    impl SegmentedVec {
        fn iter(&self) -> impl Iterator<Item=&Marble> + '_ {
            self.marbles.iter()

//            struct SegmentedVecIter<'a> {
//                vec: &'a SegmentedVec,
//                cursor: Cursor,
//            }
//
//            impl<'a> Iterator for SegmentedVecIter<'a> {
//                type Item = Marble;
//
//                fn next(&mut self) -> Option<Self::Item> {
//                    let item = self.vec.get(&self.cursor).map(|&m| m);
//                    self.cursor = self.vec.seek_forward(&self.cursor, 1);
//
//                    item
//                }
//            }
//
//            let iter = SegmentedVecIter {
//                vec: &self,
//                cursor: Cursor::default(),
//            };
//            iter.take(self.len())
        }
    }

    /// Points to a slot in the SegmentedVec, but there might not be an element there
    #[derive(Debug, PartialEq, Default)]
    struct Cursor {
        //        segment: usize,
        offset: usize,
    }

    impl SegmentedVec {
        fn len(&self) -> usize {
            self.marbles.len()
//            self.segments.iter().map(|segment| segment.len()).sum()
        }

        fn get(&self, cursor: &Cursor) -> Option<&Marble> {
            self.marbles.get(cursor.offset)
//            let segment = self.segments.get(cursor.segment)?;
//            segment.get(cursor.offset)
        }

        /// wraps around as necessary
        fn seek_forward(&self, cursor: &Cursor, steps: usize) -> Cursor {
            let new_index = (cursor.offset + steps) % self.marbles.len();
            Cursor { offset: new_index }

//            let current_segment = &self.segments[cursor.segment];
//
//            // simplest case first - the desired offset is still within the current segment
//            if cursor.offset + steps < current_segment.len() {
//                return Cursor {
//                    segment: cursor.segment,
//                    offset: cursor.offset + steps,
//                };
//            }
//
//            // otherwise, we need to move to a new segment
//            let next_segment = (cursor.segment + 1) % self.segments.len();
//            let remaining_steps = steps - (current_segment.len() - cursor.offset);
//            let intermediate_cursor = Cursor {
//                segment: next_segment,
//                offset: 0,
//            };
//
//            self.seek_forward(&intermediate_cursor, remaining_steps)
        }

        fn seek_back(&self, cursor: &Cursor, steps: usize) -> Cursor {
            let new_index = ((cursor.offset as isize) - (steps as isize) + (self.marbles.len() as isize)) as usize % self.marbles.len() as usize;
            Cursor { offset: new_index }
        }

        fn insert(&mut self, cursor: &Cursor, element: Marble) {
            self.marbles.insert(cursor.offset, element)
        }

        fn remove(&mut self, cursor: &Cursor) -> Marble {
            self.marbles.remove(cursor.offset)
        }

        fn cursor_to_index(&self, cursor: &Cursor) -> usize {
            cursor.offset
        }
    }

//    #[test]
//    fn cursor_seek_wraps_nicely() {
//        let s = SegmentedVec {
//            marbles: vec![
//                vec![Marble(0), Marble(1), Marble(2)],
//                vec![Marble(3), Marble(4)],
//            ],
//        };
//
//        assert_eq!(
//            s.seek_forward(
//                &Cursor {
//                    segment: 0,
//                    offset: 0,
//                },
//                0,
//            ),
//            Cursor {
//                segment: 0,
//                offset: 0,
//            }
//        );
//        assert_eq!(
//            s.seek_forward(
//                &Cursor {
//                    segment: 0,
//                    offset: 0,
//                },
//                1,
//            ),
//            Cursor {
//                segment: 0,
//                offset: 1,
//            }
//        );
//        assert_eq!(
//            s.seek_forward(
//                &Cursor {
//                    segment: 0,
//                    offset: 0,
//                },
//                2,
//            ),
//            Cursor {
//                segment: 0,
//                offset: 2,
//            }
//        );
//        assert_eq!(
//            s.seek_forward(
//                &Cursor {
//                    segment: 0,
//                    offset: 0,
//                },
//                3,
//            ),
//            Cursor {
//                segment: 1,
//                offset: 0,
//            }
//        );
//        assert_eq!(
//            s.seek_forward(
//                &Cursor {
//                    segment: 0,
//                    offset: 0,
//                },
//                5,
//            ),
//            Cursor {
//                segment: 0,
//                offset: 0,
//            }
//        );
//
//        assert_eq!(
//            s.seek_back(
//                &Cursor {
//                    segment: 0,
//                    offset: 2,
//                },
//                3,
//            ),
//            Cursor {
//                segment: 1,
//                offset: 1,
//            }
//        );
//    }

//    #[test]
//    fn segmented_vec_iter_behaves() {
//        let s = SegmentedVec {
//            marbles: vec![
//                vec![Marble(0), Marble(1), Marble(2)],
//                vec![Marble(3), Marble(4)],
//            ],
//        };
//
//        assert_eq!(
//            s.iter().collect_vec(),
//            vec![Marble(0), Marble(1), Marble(2), Marble(3), Marble(4)]
//        );
//    }

    #[test]
    fn sample_data_1() {
        let mut game = Game::new(9, 25);
        while game.next() {
            dbg!(&game);
        }
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
