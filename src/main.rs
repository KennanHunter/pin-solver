use arr_macro::arr;

use crate::board::BoardState;

mod board;

fn main() {
    println!("{:?}", BoardState::new(arr![true; 15]))
}
