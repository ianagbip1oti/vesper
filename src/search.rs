use crate::board::{Board, META_TURN};
use crate::movegen::{self, MoveField};
use crate::eval;
use crate::lane::Lane;

pub fn search(board: Board, depth: i32) -> MoveField {
    let mut moves = movegen::generate_moves_for_lane(&board, 0);
    if moves.is_empty() {
        return MoveField { from: Lane::EMPTY, to: Lane::EMPTY };
    }

    moves.sort_by_key(|m| {
        let to_sq = m.to.extract(0);
        let captured = (board.occupied().extract(0) & to_sq) != 0;
        if captured { -1 } else { 0 }
    });

    let packed = movegen::pack_move_fields(&moves);
    let mut best_move = moves[0];
    let mut best_score = -2000000;

    let turn_white = (board.metadata.extract(0) & (1 << META_TURN)) == 0;

    for pm in packed {
        let mut next_board = board;
        next_board.apply_move(&pm);

        let scores = vpts_recurse(&next_board, depth - 1);

        for i in 0..4 {
            let score = if turn_white { scores[i] } else { -scores[i] };
            if score > best_score {
                best_score = score;
                best_move = MoveField {
                    from: Lane::from_single(pm.from.extract(i)),
                    to: Lane::from_single(pm.to.extract(i)),
                };
            }
        }
    }

    best_move
}

fn vpts_recurse(board: &Board, depth: i32) -> [i32; 4] {
    if depth <= 0 {
        return eval::evaluate(board);
    }

    let mut froms = [0u64; 4];
    let mut tos = [0u64; 4];
    let mut any_move = false;

    for i in 0..4 {
        let moves = movegen::generate_moves_for_lane(board, i);
        if let Some(m) = moves.first() {
            froms[i] = m.from.extract(0);
            tos[i] = m.to.extract(0);
            any_move = true;
        }
    }

    if !any_move {
        return eval::evaluate(board);
    }

    let pm = MoveField {
        from: Lane::new(froms[0], froms[1], froms[2], froms[3]),
        to: Lane::new(tos[0], tos[1], tos[2], tos[3]),
    };

    let mut next_board = *board;
    next_board.apply_move(&pm);
    vpts_recurse(&next_board, depth - 1)
}
