#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::movegen;
    use crate::search;

    #[test]
    fn test_starting_position_moves() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let moves = movegen::generate_moves_for_lane(&board, 0);
        assert!(!moves.is_empty());
    }

    #[test]
    fn test_search() {
        let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let best_move = search::search(board, 2);
        assert!(best_move.from.extract(0) != 0);
        assert!(best_move.to.extract(0) != 0);
    }
}
