use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoardState([bool; 15]);

impl BoardState {
    /// Creates a new [`BoardState`].
    pub fn new(state: [bool; 15]) -> BoardState {
        BoardState(state)
    }

    fn as_bit_slice(self) -> [bool; 15] {
        self.0
    }

    pub fn normalize_orientation(self) -> BoardState {
        let data = self.as_bit_slice().map(|val| match val {
            true => 1,
            false => 0,
        });

        let tri1: u8 = data[..=5].into_iter().sum();
        let tri2 = &data[3] + &data[6] + &data[7] + &data[10] + &data[11] + &data[12];

        let tri3 = &data[5] + &data[8] + &data[9] + &data[12] + &data[13] + &data[14];

        if tri1 == tri2 && tri2 == tri3 {
            // All equal
            self
        } else if tri1 > tri2 && tri1 > tri3 {
            // Tri1 largest
            self
        } else if tri2 > tri1 && tri2 > tri3 {
            // Tri2 largest
            self.rotate_counter_clockwise()
        } else {
            // Tri3 largest
            self.rotate_clockwise()
        }
    }

    pub fn rotate_clockwise(self) -> BoardState {
        let data = self.as_bit_slice();

        BoardState([
            data[14], data[9], data[13], data[5], data[8], data[12], data[2], data[4], data[7],
            data[11], data[0], data[1], data[3], data[6], data[10],
        ])
    }

    pub fn rotate_counter_clockwise(self) -> BoardState {
        let data = self.as_bit_slice();

        BoardState([
            data[10], data[11], data[6], data[12], data[7], data[3], data[13], data[8], data[4],
            data[1], data[14], data[9], data[5], data[2], data[0],
        ])
    }
}

impl fmt::Debug for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.clone())
    }
}

impl fmt::Display for BoardState {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arr_macro::arr;

    #[test]
    fn test_board_creation() {
        assert_eq!(
            BoardState::new(arr![false; 15]),
            BoardState(arr![false; 15])
        )
    }

    #[test]
    fn test_normalize_orientation() {
        let simple_correct_orientation = BoardState::new([
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ]);

        assert_eq!(
            simple_correct_orientation,
            simple_correct_orientation.normalize_orientation()
        );

        let clockwise = simple_correct_orientation.rotate_clockwise();
        let counter_clockwise = simple_correct_orientation.rotate_counter_clockwise();

        assert_eq!(
            clockwise,
            BoardState::new([
                false, false, false, false, false, false, false, false, false, false, true, false,
                false, false, false,
            ])
        );

        assert_eq!(
            counter_clockwise,
            BoardState::new([
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, true,
            ])
        );

        assert_eq!(clockwise.rotate_clockwise(), counter_clockwise);

        assert_eq!(
            simple_correct_orientation.normalize_orientation(),
            counter_clockwise.normalize_orientation(),
        );
        assert_eq!(
            simple_correct_orientation.normalize_orientation(),
            clockwise.normalize_orientation(),
        );
    }
}
