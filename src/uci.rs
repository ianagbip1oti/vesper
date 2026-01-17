use std::io;
use crate::board::Board;
use crate::search;
use crate::movegen::MoveField;

pub fn main_loop() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let stdin = io::stdin();
    for line in stdin.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "uci" => {
                println!("id name Vesper");
                println!("id author Jules");
                println!("uciok");
            }
            "isready" => println!("readyok"),
            "ucinewgame" => {
                board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            }
            "position" => {
                if parts.len() > 1 {
                    if parts[1] == "startpos" {
                        board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
                        if parts.len() > 2 && parts[2] == "moves" {
                            apply_uci_moves(&mut board, &parts[3..]);
                        }
                    } else if parts[1] == "fen" {
                        // Reconstruct FEN
                        let fen_parts = &parts[2..8];
                        let fen = fen_parts.join(" ");
                        board = Board::from_fen(&fen);
                        if parts.len() > 8 && parts[8] == "moves" {
                            apply_uci_moves(&mut board, &parts[9..]);
                        }
                    }
                }
            }
            "go" => {
                let best_move = search::search(board, 4);
                println!("bestmove {}", move_to_uci(&best_move));
            }
            "quit" => break,
            _ => {}
        }
    }
}

fn apply_uci_moves(board: &mut Board, moves: &[&str]) {
    for m_str in moves {
        // Find the move in legal moves
        let legal_moves = crate::movegen::generate_moves_for_lane(board, 0);
        for m in legal_moves {
            if move_to_uci(&m) == *m_str {
                board.apply_move(&m);
                break;
            }
        }
    }
}

fn sq_to_uci(sq: u32) -> String {
    let file = (sq % 8) as u8 + b'a';
    let rank = (sq / 8) as u8 + b'1';
    format!("{}{}", file as char, rank as char)
}

fn move_to_uci(m: &MoveField) -> String {
    let f = m.from.extract(0);
    let t = m.to.extract(0);
    if f == 0 || t == 0 { return "0000".to_string(); }
    let from = f.trailing_zeros();
    let to = t.trailing_zeros();
    format!("{}{}", sq_to_uci(from), sq_to_uci(to))
}
