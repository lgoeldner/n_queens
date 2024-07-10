use core::fmt;
use std::{iter, time::Instant};

fn main() {
    let res = {
        let time = Instant::now();
        let res = n_queens::<25>();
        println!("took {:?}", time.elapsed());
        res
    }
    .map(Board);
    dbg!(&res);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Tile {
    Queen,
    #[default]
    Empty,
}

struct Board<const N: usize>(pub [[Tile; N]; N]);

impl<const N: usize> fmt::Debug for Board<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for r in 0..N {
            for c in 0..N {
                let ch = match self.0[r][c] {
                    Tile::Empty => '.',
                    Tile::Queen => 'Q',
                };

                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[test]
fn test_canset() {
    use Tile::Empty as E;
    use Tile::Queen as Q;
    let b = [
        [Q, E, E, E], //
        [E, E, E, E],
        [E, E, E, E],
        [E, E, E, E],
    ];

    assert!(can_set_queen(&b, (1, 2)));
    assert!(can_set_queen(&b, (2, 1)));
}

fn can_set_queen<const N: usize>(board: &[[Tile; N]; N], (row, col): (usize, usize)) -> bool {
    // check each direction if theres a queen there
    // left/right doesnt get checked because theres only one queen added per row

    let left_up_iter = (0..row).rev().zip((0..col).rev());
    let left_down_iter = (row..N).zip((0..col).rev());
    let up_down_iter = (0..row).zip(iter::repeat(col));

    left_up_iter
        .chain(left_down_iter)
        .chain(up_down_iter)
        .map(|(r, c)| board[r][c])
        .all(|it| !(it == Tile::Queen))
}

fn n_queens<const N: usize>() -> Result<[[Tile; N]; N], ()> {
    let mut board = [[Default::default(); N]; N];
    do_n_queens(&mut board, 0).map(|_| board)
}

fn do_n_queens<const N: usize>(board: &mut [[Tile; N]; N], row: usize) -> Result<(), ()> {
    if row == N {
        return Ok(());
    }

    // iterate throug the row were interested in
    // check if a position is possible
    // if it is, do a recursive call to get the next position
    for pos in 0..N {
        if can_set_queen(board, (row, pos)) {
            // let old_board = board;
            board[row][pos] = Tile::Queen;
            match do_n_queens(board, row + 1) {
                Err(_) => {
                    board[row][pos] = Tile::Empty;
                    continue;
                }
                Ok(board) => return Ok(board),
            }
        }
    }

    Err(())
}
