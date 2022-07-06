use wasm_bindgen::prelude::*;
use connect4::Connect4;
use mcts::GeneralGame;
use mcts::Node;

mod connect4;
mod mcts;

#[wasm_bindgen]
pub fn predict_best_move(board: &str, player:i8, iters: usize) -> Option<usize> {
    let board_opt = Connect4::<6,7,4>::from_string(board);

    if let Some(board) = board_opt {
        let mut root_node = Node::<Connect4<6,7,4>>::new(board, player, 0);

        root_node.predict(iters, 1);

        let child = root_node.get_most_visited_child();
        if let Some(child) = child {
            return Some(child.move_index);
        }
    }

    return None;
}

#[wasm_bindgen]
pub fn get_score(board: &str) -> Option<i8> {
    let board_opt = Connect4::<6,7,4>::from_string(board);

    if let Some(board) = board_opt {
        return Some(board.get_score());
    }

    return None;
}


#[cfg(test)]
mod tictactoe;

#[cfg(test)]
mod tests {
    use crate::predict_best_move;

    #[test]
    fn best_move_test() {
        let board = "\
                            .......\n\
                            .......\n\
                            .......\n\
                            .......\n\
                            ......O\n\
                            .OXX.XO\n\
                        ";
        assert_eq!(predict_best_move(board, 1, 100), Some(4));
        assert_eq!(predict_best_move(board, -1, 100), Some(4));

        let board = "\
                            .......\n\
                            .......\n\
                            .......\n\
                            O.....X\n\
                            O.....X\n\
                            O.....X\n\
                        ";
        assert_eq!(predict_best_move(board, 1, 100), Some(6));
    }
}