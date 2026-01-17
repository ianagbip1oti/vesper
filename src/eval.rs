use crate::board::Board;
use crate::movegen;

pub fn evaluate(board: &Board) -> [i32; 4] {
    let mut scores = [0i32; 4];

    // Mobility plane (parallel)
    let white_attacks = movegen::get_attacks(board, board.white, true);
    let black_attacks = movegen::get_attacks(board, board.black, false);

    for i in 0..4 {
        let mut score = 0;

        let w_pawns = (board.pawns & board.white).extract(i).count_ones() as i32;
        let b_pawns = (board.pawns & board.black).extract(i).count_ones() as i32;
        let w_leapers = (board.leapers & board.white).extract(i).count_ones() as i32;
        let b_leapers = (board.leapers & board.black).extract(i).count_ones() as i32;
        let w_sliders = (board.sliders & board.white).extract(i).count_ones() as i32;
        let b_sliders = (board.sliders & board.black).extract(i).count_ones() as i32;
        let w_kings = (board.kings & board.white).extract(i).count_ones() as i32;
        let b_kings = (board.kings & board.black).extract(i).count_ones() as i32;

        score += (w_pawns - b_pawns) * 100;
        score += (w_leapers - b_leapers) * 300;
        score += (w_sliders - b_sliders) * 500;
        score += (w_kings - b_kings) * 10000;

        // Mobility
        let w_mobility = white_attacks.extract(i).count_ones() as i32;
        let b_mobility = black_attacks.extract(i).count_ones() as i32;
        score += (w_mobility - b_mobility) * 5;

        // Tension
        let tension = (white_attacks & black_attacks).extract(i).count_ones() as i32;
        score += tension * 2;

        // Side to move bonus
        let meta = board.metadata.extract(i);
        let white_turn = (meta & (1 << crate::board::META_TURN)) == 0;
        if white_turn {
            score += 10;
        } else {
            score -= 10;
        }

        scores[i] = score;
    }

    scores
}
