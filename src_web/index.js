import {EMPTY_BOARD, init_wasm, check_score, get_next_best_move} from "./board.js";
let board;

function get_color(player){
    if (player==-1) {return "blue"}
    else if (player==1) {return "red"}
    else console.error("Invalid player ", player);
}

$("#board").ready(function() {   
    for (let row = 0; row < 6; row++){
        for (let column = 0; column < 7; column++){
            let elem = $("<span></span>").text(`${row+1}/${column+1}`);
            elem.append($("<span></span>"))

            elem.click(function() {clicked_board(column)})

            elem.hover(function() {hover_column(column);}, function() {hover_column_end(column);})

            elem.attr("id", `square${row}${column}`);
            $("#board").append(elem);
        }
    }
});

$(document).ready(function() {
    board = EMPTY_BOARD;
    changePlayer(player);

    
    init_wasm().then(function() {
        console.log("WASM is ready!");
    });
    
});

function drop_token(column, player) {
    const row = board.top[column];

    if (row >=0 ) {
        board.entries[row][column] = player;
        $(`#square${row}${column}`).attr("token_placed", "");
        $(`#square${row}${column}`).attr("token", get_color(player));
        $(`#square${row}${column}`).removeAttr("hover");

        board.top[column]--;

        return true;
    }
    else {
        console.warn("Cannot add any more tokens to column ", column);

        return false;
    }
}

function hover_column(column){
    const top = board.top[column];
    if (top >= 0) {
        $(`#square${top}${column}`).attr('hover', "")
    }
}
function hover_column_end(column){
    const top = board.top[column];
    $(`#square${top}${column}`).removeAttr('hover')
}

function changePlayer(player){
    $('#board>span').not('[token_placed]').attr('token', get_color(player));
}


let player = 1;
/*function clicked_board(column) {
    
    drop_token(column, player)

    player *= -1;
    changePlayer(player);

    const board_str = get_board_string(board);
    let best_move = predict_best_move(board_str, player, 300);
    console.log("Best move is: ", best_move+1);
}*/
function clicked_board(column) {
    
    drop_token(column, player)

    let score = check_score(board);
    if (score == 1) { setTimeout(function() { alert("Red won!"); }, 300); return; }
    if (score == -1) { setTimeout(function() { alert("Blue won!"); }, 300); return; }

    let best_move = get_next_best_move(board, -player, 300);

    drop_token(best_move, -player) 
    score = check_score(board);
    if (score == 1) { setTimeout(function() { alert("Red won!"); }, 300); return; }
    if (score == -1) { setTimeout(function() { alert("Blue won!"); }, 300); return; }
}