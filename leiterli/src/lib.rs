use std::cmp;
use rand::Rng;

const SIZE: usize = 130;

pub struct Board {
    jump_to: [usize; SIZE]
}

impl Board {
    pub fn new(jumps: &[(usize, usize)]) -> Self {
        // empty board
        let mut arr = [0; SIZE];
        for i in 0..SIZE {
            arr[i] = i;
        }
        // insert all jumps
        for j in jumps {
            arr[j.0] = j.1;
        }
        // validate jumps
        for (i, &x) in arr.iter().enumerate() {
            // the jump-from field must be on the board
            if i >= SIZE {
                panic!("cannot jump from {:?} because outside of board.", i);
            }
            // the jump-to field must be on the board
            if x >= SIZE {
                panic!("cannot jump to {:?} because outside of board.", x);
            }
            // the jump-to field must be non-jumping field
            if arr[x] != x {
                panic!("jumping from {:?} to {:?}, cannot jump from there.", i, x);
            }
        }
        // create actual board
        Board {
            jump_to: arr
        }
    }
}

impl Board {
    pub fn jump(&self, from: usize) -> usize {
        return self.jump_to[from];
    }
}

pub struct Play<'a> {
    board: &'a Board,
    pub position: usize,
    pub moves: usize
}

impl<'a> Play<'a> {
    pub fn new(b: &'a Board) -> Self {
        Play {
            board: b,
            position: 0,
            moves: 0
        }
    }
}

impl<'a> Play<'a> {
    pub fn move_by(&mut self, steps: usize) {
        let p = self.bounce(steps);
        self.position = self.board.jump(p);
        self.moves += 1;
    }
}

impl<'a> Play<'a> {
    pub fn done(&self) -> bool {
        if self.position == SIZE-1 {
            return true;
        }
        else {
            return false;
        }
    }
}

impl<'a> Play<'a> {
    fn bounce(&self, steps: usize) -> usize {
        let mut p = (self.position + steps) as i32;
        let corr = 2 * cmp::min(SIZE as i32 - 1 - p, 0);
        p += corr;
        return p as usize;
    }
}

pub struct Dice {
    rng: rand::rngs::ThreadRng,
    lower: usize,
    upper: usize
}

impl Dice {
    pub fn new(l: usize, u: usize) -> Self {
        Dice {
            rng: rand::thread_rng(),
            lower: l,
            upper: u
        }
    }
}

impl Dice {
    pub fn throw(&mut self) -> usize {
        return self.rng.gen_range(self.lower, self.upper+1);
    }
}


#[test]
fn board_without_jumps() {
    let b = Board::new(&[]);
    assert_eq!(b.jump(0), 0);
    assert_eq!(b.jump(37), 37);
    assert_eq!(b.jump(99), 99);
}

#[test]
fn board_with_jumps() {
    let b = Board::new(&[(0, 10), (37, 25)]);
    assert_eq!(b.jump(0), 10);
    assert_eq!(b.jump(37), 25);
    assert_eq!(b.jump(99), 99);
}

#[test]
#[should_panic]
fn board_with_invalid_jumps_1() {
    let _b = Board::new(&[(99, 10), (10, 3)]);
}

#[test]
#[should_panic]
fn board_with_invalid_jumps_2() {
    let _b = Board::new(&[(99, 130)]);
}

#[test]
#[should_panic]
fn board_with_invalid_jumps_3() {
    let _b = Board::new(&[(130, 1)]);
}

#[test]
fn play_without_jumps() {
    let b = Board::new(&[]);
    let mut p = Play::new(&b);
    assert_eq!(p.position, 0);
    p.move_by(0);
    assert_eq!(p.position, 0);
    p.move_by(3);
    assert_eq!(p.position, 3);
    assert_eq!(p.moves, 2);
}

#[test]
fn play_with_jumps() {
    let b = Board::new(&[(0, 3), (5, 1)]);
    let mut p = Play::new(&b);
    assert_eq!(p.position, 0);
    p.move_by(0);
    assert_eq!(p.position, 3);
    p.move_by(1);
    assert_eq!(p.position, 4);
    p.move_by(1);
    assert_eq!(p.position, 1);
    assert_eq!(p.moves, 3);
}

#[test]
fn play_with_jumps_at_end() {
    let b = Board::new(&[(1, 125)]);
    let mut p = Play::new(&b);
    p.move_by(1);
    assert_eq!(p.position, 125);
    p.move_by(6);
    assert_eq!(p.position, 127);
    assert_eq!(p.moves, 2);
}

#[test]
fn play_to_end() {
    let b = Board::new(&[(1, 129)]);
    let mut p = Play::new(&b);
    assert_eq!(p.done(), false);
    p.move_by(1);
    assert_eq!(p.done(), true);
    assert_eq!(p.moves, 1);
}

#[test]
fn throw_dice() {
    let mut d = Dice::new(1, 6);
    let i = d.throw();
    assert!(i >= 1);
    assert!(i < 7);
}