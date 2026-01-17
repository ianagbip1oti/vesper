use crate::lane::Lane;
use crate::movegen::MoveField;

#[derive(Clone, Copy)]
pub struct Board {
    pub pawns: Lane,
    pub leapers: Lane,
    pub sliders: Lane,
    pub kings: Lane,

    pub white: Lane,
    pub black: Lane,

    pub diagonal: Lane,   // Trait: Bishops and Queens
    pub orthogonal: Lane, // Trait: Rooks and Queens

    pub metadata: Lane, // [Meta1, Meta2, Meta3, Meta4]
}

// Metadata bit offsets
pub const META_TURN: u64 = 0; // 1 bit: 0 = white, 1 = black
pub const META_CASTLING: u64 = 1; // 4 bits
pub const META_EP: u64 = 5; // 7 bits (0-63, 64 for none)

impl Board {
    pub fn new_empty() -> Self {
        Self {
            pawns: Lane::EMPTY,
            leapers: Lane::EMPTY,
            sliders: Lane::EMPTY,
            kings: Lane::EMPTY,
            white: Lane::EMPTY,
            black: Lane::EMPTY,
            diagonal: Lane::EMPTY,
            orthogonal: Lane::EMPTY,
            metadata: Lane::EMPTY,
        }
    }

    pub fn occupied(&self) -> Lane {
        self.pawns | self.leapers | self.sliders | self.kings
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Self::new_empty();
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let rows: Vec<&str> = parts[0].split('/').collect();

        let mut w_pawns = 0u64;
        let mut w_knights = 0u64;
        let mut w_bishops = 0u64;
        let mut w_rooks = 0u64;
        let mut w_queens = 0u64;
        let mut w_king = 0u64;

        let mut b_pawns = 0u64;
        let mut b_knights = 0u64;
        let mut b_bishops = 0u64;
        let mut b_rooks = 0u64;
        let mut b_queens = 0u64;
        let mut b_king = 0u64;

        for (i, row) in rows.iter().enumerate() {
            let rank = 7 - i;
            let mut file = 0;
            for c in row.chars() {
                if let Some(digit) = c.to_digit(10) {
                    file += digit;
                } else {
                    let square = rank * 8 + file as usize;
                    let bit = 1u64 << square;
                    match c {
                        'P' => w_pawns |= bit,
                        'N' => w_knights |= bit,
                        'B' => w_bishops |= bit,
                        'R' => w_rooks |= bit,
                        'Q' => w_queens |= bit,
                        'K' => w_king |= bit,
                        'p' => b_pawns |= bit,
                        'n' => b_knights |= bit,
                        'b' => b_bishops |= bit,
                        'r' => b_rooks |= bit,
                        'q' => b_queens |= bit,
                        'k' => b_king |= bit,
                        _ => {}
                    }
                    file += 1;
                }
            }
        }

        board.pawns = Lane::from_single(w_pawns | b_pawns);
        board.leapers = Lane::from_single(w_knights | b_knights);
        board.sliders = Lane::from_single(w_bishops | b_bishops | w_rooks | b_rooks | w_queens | b_queens);
        board.kings = Lane::from_single(w_king | b_king);

        board.white = Lane::from_single(w_pawns | w_knights | w_bishops | w_rooks | w_queens | w_king);
        board.black = Lane::from_single(b_pawns | b_knights | b_bishops | b_rooks | b_queens | b_king);

        board.diagonal = Lane::from_single(w_bishops | b_bishops | w_queens | b_queens);
        board.orthogonal = Lane::from_single(w_rooks | b_rooks | w_queens | b_queens);

        let mut meta = 0u64;
        if parts.len() > 1 && parts[1] == "b" {
            meta |= 1 << META_TURN;
        }
        board.metadata = Lane::from_single(meta);

        board
    }

    pub fn apply_move(&mut self, mv: &MoveField) {
        let from = mv.from;
        let to = mv.to;

        let is_pawn = (self.pawns & from).is_not_zero_mask();
        let is_leaper = (self.leapers & from).is_not_zero_mask();
        let is_slider = (self.sliders & from).is_not_zero_mask();
        let is_king = (self.kings & from).is_not_zero_mask();

        let is_diag = (self.diagonal & from).is_not_zero_mask();
        let is_ortho = (self.orthogonal & from).is_not_zero_mask();

        let was_white = (self.white & from).is_not_zero_mask();
        let was_black = (self.black & from).is_not_zero_mask();

        // Remove piece from 'from'
        let not_from = !from;
        self.pawns &= not_from;
        self.leapers &= not_from;
        self.sliders &= not_from;
        self.kings &= not_from;
        self.white &= not_from;
        self.black &= not_from;
        self.diagonal &= not_from;
        self.orthogonal &= not_from;

        // Remove captured piece from 'to'
        let not_to = !to;
        self.pawns &= not_to;
        self.leapers &= not_to;
        self.sliders &= not_to;
        self.kings &= not_to;
        self.white &= not_to;
        self.black &= not_to;
        self.diagonal &= not_to;
        self.orthogonal &= not_to;

        // Place piece at 'to'
        self.pawns |= to & is_pawn;
        self.leapers |= to & is_leaper;
        self.sliders |= to & is_slider;
        self.kings |= to & is_king;
        self.white |= to & was_white;
        self.black |= to & was_black;
        self.diagonal |= to & is_diag;
        self.orthogonal |= to & is_ortho;

        // Update turn (flip bit)
        self.metadata ^= Lane::from_single(1 << META_TURN);
    }
}
