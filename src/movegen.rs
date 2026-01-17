use crate::lane::Lane;
use crate::board::Board;

#[derive(Clone, Copy, Debug)]
pub struct MoveField {
    pub from: Lane,
    pub to: Lane,
}

pub fn get_attacks(board: &Board, us: Lane, is_white: bool) -> Lane {
    let occupied = board.occupied();
    let empty = !occupied;

    let pawns = board.pawns & us;
    let leapers = board.leapers & us;
    let sliders = board.sliders & us;
    let kings = board.kings & us;

    let mut attacks = Lane::EMPTY;

    if is_white {
        attacks |= pawns.shift_north_east() | pawns.shift_north_west();
    } else {
        attacks |= pawns.shift_south_east() | pawns.shift_south_west();
    }

    attacks |= leapers.knight_attacks();

    let diag = sliders & board.diagonal;
    let ortho = sliders & board.orthogonal;

    attacks |= diag.fill_north_east(empty).shift_north_east();
    attacks |= diag.fill_north_west(empty).shift_north_west();
    attacks |= diag.fill_south_east(empty).shift_south_east();
    attacks |= diag.fill_south_west(empty).shift_south_west();

    attacks |= ortho.fill_north(empty).shift_north();
    attacks |= ortho.fill_south(empty).shift_south();
    attacks |= ortho.fill_east(empty).shift_east();
    attacks |= ortho.fill_west(empty).shift_west();

    attacks |= kings.king_attacks();

    attacks
}

pub fn generate_moves_for_lane(board: &Board, lane_idx: usize) -> Vec<MoveField> {
    let mut move_fields = Vec::new();
    let occupied = board.occupied().extract(lane_idx);
    let empty = !occupied;

    let meta = board.metadata.extract(lane_idx);
    let white_turn = (meta & (1 << crate::board::META_TURN)) == 0;

    let us = if white_turn { board.white.extract(lane_idx) } else { board.black.extract(lane_idx) };
    let them = if white_turn { board.black.extract(lane_idx) } else { board.white.extract(lane_idx) };
    let pawns = board.pawns.extract(lane_idx) & us;
    let leapers = board.leapers.extract(lane_idx) & us;
    let sliders = board.sliders.extract(lane_idx) & us;
    let kings = board.kings.extract(lane_idx) & us;

    // Pawns
    let mut p = pawns;
    while p != 0 {
        let from_bit = 1 << p.trailing_zeros();
        let targets = if white_turn { (from_bit << 8) & empty } else { (from_bit >> 8) & empty };
        add_moves(&mut move_fields, from_bit, targets);

        if white_turn {
            if (from_bit & 0x000000000000ff00) != 0 {
                let push1 = from_bit << 8;
                if (push1 & empty) != 0 {
                    let push2 = push1 << 8;
                    if (push2 & empty) != 0 {
                        add_moves(&mut move_fields, from_bit, push2);
                    }
                }
            }
        } else {
            if (from_bit & 0x00ff000000000000) != 0 {
                let push1 = from_bit >> 8;
                if (push1 & empty) != 0 {
                    let push2 = push1 >> 8;
                    if (push2 & empty) != 0 {
                        add_moves(&mut move_fields, from_bit, push2);
                    }
                }
            }
        }

        let caps = if white_turn {
            ((from_bit << 7) & !0x8080808080808080u64 & them) | ((from_bit << 9) & !0x0101010101010101u64 & them)
        } else {
            ((from_bit >> 7) & !0x0101010101010101u64 & them) | ((from_bit >> 9) & !0x8080808080808080u64 & them)
        };
        add_moves(&mut move_fields, from_bit, caps);
        p &= p - 1;
    }

    let mut l = leapers;
    while l != 0 {
        let from_bit = 1 << l.trailing_zeros();
        let targets = Lane::from_single(from_bit).knight_attacks().extract(0) & !us;
        add_moves(&mut move_fields, from_bit, targets);
        l &= l - 1;
    }

    let mut s = sliders;
    while s != 0 {
        let from_bit = 1 << s.trailing_zeros();
        let diag = from_bit & board.diagonal.extract(lane_idx);
        let ortho = from_bit & board.orthogonal.extract(lane_idx);

        let mut targets = 0u64;
        let l_from = Lane::from_single(from_bit);
        let l_empty = Lane::from_single(empty);

        if diag != 0 {
            targets |= (l_from.fill_north_east(l_empty).shift_north_east() |
                        l_from.fill_north_west(l_empty).shift_north_west() |
                        l_from.fill_south_east(l_empty).shift_south_east() |
                        l_from.fill_south_west(l_empty).shift_south_west()).extract(0);
        }
        if ortho != 0 {
            targets |= (l_from.fill_north(l_empty).shift_north() |
                        l_from.fill_south(l_empty).shift_south() |
                        l_from.fill_east(l_empty).shift_east() |
                        l_from.fill_west(l_empty).shift_west()).extract(0);
        }
        add_moves(&mut move_fields, from_bit, targets & !us);
        s &= s - 1;
    }

    let mut k = kings;
    while k != 0 {
        let from_bit = 1 << k.trailing_zeros();
        let targets = Lane::from_single(from_bit).king_attacks().extract(0) & !us;
        add_moves(&mut move_fields, from_bit, targets);
        k &= k - 1;
    }

    move_fields
}

fn add_moves(moves: &mut Vec<MoveField>, from_bit: u64, targets: u64) {
    let mut t = targets;
    while t != 0 {
        let to_bit = 1 << t.trailing_zeros();
        moves.push(MoveField {
            from: Lane::from_single(from_bit),
            to: Lane::from_single(to_bit),
        });
        t &= t - 1;
    }
}

pub fn pack_move_fields(moves: &[MoveField]) -> Vec<MoveField> {
    if moves.is_empty() { return Vec::new(); }
    let mut packed = Vec::new();
    for i in (0..moves.len()).step_by(4) {
        let m1 = moves[i];
        let m2 = moves.get(i+1).unwrap_or(&m1);
        let m3 = moves.get(i+2).unwrap_or(&m1);
        let m4 = moves.get(i+3).unwrap_or(&m1);

        packed.push(MoveField {
            from: Lane::new(m1.from.extract(0), m2.from.extract(0), m3.from.extract(0), m4.from.extract(0)),
            to: Lane::new(m1.to.extract(0), m2.to.extract(0), m3.to.extract(0), m4.to.extract(0)),
        });
    }
    packed
}
