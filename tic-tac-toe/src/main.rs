use std::{
    f32::{INFINITY, NEG_INFINITY},
    fmt::Display,
    io,
    ops::{Add, Sub},
};

fn main() {}

fn play_engine(side: Side) {
    let mut game = Game::init();
    loop {
        if game.board.side_to_move == side {
            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buffer).unwrap();
            game.manual_move(buffer[0..1].parse::<u8>().unwrap());
        } else {
            game.engine_move();
        }
        println!("{}", game.board);
        if game.board.is_winning(Side::X) {
            println!("Game over, X has won!");
            break;
        } else if game.board.is_winning(Side::O) {
            println!("Game over, O has won!");
            break;
        }
        if game.board.get_empty().0 == 0 {
            println!("Game over, result is draw!");
            break;
        }
    }
}

const WINNING_STATES: [u16; 8] = [
    448, // 111000000
    56,  // 000111000
    7,   // 000000111
    292, // 100100100
    146, // 010010010
    73,  // 001001001
    273, // 100010001
    84,  // 001010100
];
const FULLBAORD: BitBoard = BitBoard(511); // 111111111

#[derive(Clone, Copy, PartialEq)]
enum Side {
    X,
    O,
}
impl Side {
    pub fn other(&self) -> Self {
        match self {
            Side::X => Side::O,
            Side::O => Side::X,
        }
    }
}

pub struct Game {
    board: Board,
}
impl Game {
    pub fn init() -> Self {
        Self {
            board: Board::initiate(),
        }
    }
    pub fn engine_move(&mut self) {
        let engine = Engine::from(self.board);
        if let Some(m) = engine.execute() {
            self.board = m;
        } else {
            println!("Game over!");
        }
    }
    pub fn manual_move(&mut self, square: u8) {
        self.board.make_move(square);
    }
}

#[derive(Clone, Copy)]
struct Board {
    x_side: BitBoard,
    o_side: BitBoard,
    side_to_move: Side,
}
impl Board {
    pub fn new(x_side: BitBoard, o_side: BitBoard, side_to_move: Side) -> Self {
        Board {
            x_side,
            o_side,
            side_to_move,
        }
    }

    pub fn make_move(&mut self, square: u8) {
        if (1 << square) & self.get_empty().0 != 0 {
            if self.side_to_move == Side::X {
                self.x_side.0 |= 1 << square
            } else if self.side_to_move == Side::O {
                self.o_side.0 |= 1 << square
            }
            self.side_to_move = self.side_to_move.other();
        }
    }

    pub fn initiate() -> Self {
        Board::new(BitBoard(0), BitBoard(0), Side::X)
    }

    fn generate_moves(&self) -> Vec<Board> {
        let empty_squares = self.get_empty();
        let side = self.get_side_bitboard().0;
        let mut res = Vec::new();
        for n in 0..9 {
            if (1 << n) & empty_squares.0 != 0 {
                let new_side = side | (1 << n);
                match self.side_to_move {
                    Side::X => res.push(Board::new(
                        BitBoard(new_side),
                        self.o_side,
                        self.side_to_move.other(),
                    )),
                    Side::O => res.push(Board::new(
                        self.x_side,
                        BitBoard(new_side),
                        self.side_to_move.other(),
                    )),
                }
            }
        }
        res
    }

    fn get_side_bitboard(&self) -> BitBoard {
        match self.side_to_move {
            Side::X => self.x_side,
            Side::O => self.o_side,
        }
    }

    fn get_empty(&self) -> BitBoard {
        FULLBAORD - (self.x_side + self.o_side)
    }

    pub fn is_winning(&self, side: Side) -> bool {
        let side_bitboard = match side {
            Side::X => self.x_side,
            Side::O => self.o_side,
        };

        for state in WINNING_STATES {
            if state & side_bitboard.0 == state {
                return true;
            }
        }
        false
    }
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..3 {
            for j in 0..3 {
                let n = 3 * i + j;
                if self.x_side.0 & (1 << n) != 0 {
                    write!(f, "X ")?;
                } else if self.o_side.0 & (1 << n) != 0 {
                    write!(f, "O ")?;
                } else {
                    write!(f, "_ ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct BitBoard(u16);
impl Sub<Self> for BitBoard {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 - rhs.0)
    }
}
impl Add<Self> for BitBoard {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 + rhs.0)
    }
}

struct Engine {
    board: Board,
}
impl Engine {
    pub fn execute(&self) -> Option<Board> {
        let moves = self.board.generate_moves();
        let mut evaluated_moves = Vec::new();
        let maximize = self.board.side_to_move == Side::X;
        for m in &moves {
            evaluated_moves.push((m, Self::minimax(*m, !maximize)));
        }

        if maximize {
            if let Some(m) = evaluated_moves
                .iter()
                .max_by(|b1, b2| b1.1.partial_cmp(&b2.1).unwrap())
            {
                Some(*m.0)
            } else {
                None
            }
        } else {
            if let Some(m) = evaluated_moves
                .iter()
                .min_by(|b1, b2| b1.1.partial_cmp(&b2.1).unwrap())
            {
                Some(*m.0)
            } else {
                None
            }
        }
    }

    pub fn minimax(board: Board, maximize: bool) -> f32 {
        if board.is_winning(Side::X) {
            return f32::MAX;
        } else if board.is_winning(Side::O) {
            return f32::MIN;
        }

        let next_gen = board.generate_moves();
        if next_gen.len() == 0 {
            return 0.;
        }

        if maximize {
            let mut val = NEG_INFINITY;
            for child in next_gen {
                val = val.max(Self::minimax(child, false));
            }
            val
        } else {
            let mut val = INFINITY;
            for child in next_gen {
                val = val.min(Self::minimax(child, true));
            }
            val
        }
    }
}
impl From<Board> for Engine {
    fn from(value: Board) -> Self {
        Self { board: value }
    }
}
