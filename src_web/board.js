import init, {get_score, predict_best_move} from '../pkg/connect4_mcts.js';

export async function init_wasm(){
    await init();
}

export const EMPTY_BOARD = {
    entries: [[0, 0, 0, 0, 0, 0, 0],[0, 0, 0, 0, 0, 0, 0],[0, 0, 0, 0, 0, 0, 0],[0, 0, 0, 0, 0, 0, 0],[0, 0, 0, 0, 0, 0, 0],[0, 0, 0, 0, 0, 0, 0]],
    top: [5,5,5,5,5,5,5]
}

function get_board_string(board){
    let s = "";
    for (let row = 0; row < 6; row++) {
        for (let col = 0; col < 7; col++) {
            if (board.entries[row][col] == 0) {s += "."}
            else if (board.entries[row][col] == 1) {s += "X"}
            else if (board.entries[row][col] == -1) {s += "O"}
        }
        s += "\n"
    }

    return s;
}

export function get_next_best_move(board, player, iters) {
    const board_str = get_board_string(board);
    let best_move = predict_best_move(board_str, player, iters);

    return best_move;
}

export function check_score(board) {
    const board_str = get_board_string(board);
    let score = get_score(board_str);

    return score;
}