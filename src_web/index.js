import { empty_board, init_wasm, check_score, get_next_best_move } from "./board.js";
let board;

const HUMAN = 1;
const COMPUTER = -1;

let player = HUMAN;
let active_game = true;

let mcts_iters = 200;

let human_score = 0;
let bot_score = 0;

let current_bot;

const BOTS = ['veronica', 'anna', 'jan', 'arnie', 'puffy']

function get_bot_difficulty(bot){
    if (bot == 'veronica') return 10000;
    if (bot == 'anna') return 3000;
    if (bot == 'jan') return 800;
    if (bot == 'arnie') return 200;
    if (bot == 'puffy') return 50;
}

function get_color(player) {
    if (player == HUMAN) { return "red" }
    else if (player == COMPUTER) { return "blue" }
    else console.error("Invalid player ", player);
}

$("#board").ready(function () {
    for (let row = 0; row < 6; row++) {
        for (let column = 0; column < 7; column++) {
            let elem = $("<span></span>")//.text(`${row+1}/${column+1}`);
            elem.append($("<span></span>"))

            elem.click(function () { clicked_board(column) })

            elem.hover(function () { hover_column(column); }, function () { hover_column_end(column); })

            elem.attr("id", `square${row}${column}`);
            $("#board").append(elem);
        }
    }
});

$(document).ready(function () {
    board = empty_board();
    change_player(player);

    init_wasm().then(function () {
        console.log("WASM is ready!");
    });

    $('#new-game').click(new_game);

    $('#change-bot').click(change_bot_open);
    $('#change-bot-dialog').on('click', function (e) {
        if (e.target !== this)
            return;

        change_bot_close();

    });

    for (let i = 0; i < BOTS.length; i++){
        const bot = BOTS[i];
        $(`#select-bot-${bot}`).click(() => {select_bot_click(bot)});
    }

    select_bot_click('arnie');

    $('#result-popup').on('click', function (e) {
        if (e.target !== this)
            return;

        result_popup_close();
    });
    $('#result-ok').click(result_popup_close);

    result_popup_close();
});

function drop_token(column, player) {
    const row = board.top[column];

    if (row >= 0) {
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

function hover_column(column) {
    const top = board.top[column];
    if (top >= 0 && player == HUMAN && active_game) {
        $(`#square${top}${column}`).attr('hover', "")
    }
}
function hover_column_end(column) {
    const top = board.top[column];
    $(`#square${top}${column}`).removeAttr('hover')
}

function change_player(player) {
    $('#board>span').not('[token_placed]').attr('token', get_color(player));
}

function clicked_board(column) {
    if (player == COMPUTER || !active_game) return;

    drop_token(column, HUMAN);

    if (check_score_gui()) return;
    player = COMPUTER;

    setTimeout(computer_moves, 1000);
}

async function computer_moves() {
    let best_move = get_next_best_move(board, COMPUTER, mcts_iters);
    drop_token(best_move, COMPUTER)
    check_score_gui();
    player = HUMAN
}

function check_score_gui() {
    if (!active_game) return;
    let score = check_score(board);

    if (score == HUMAN) { active_game = false; }
    if (score == COMPUTER) { active_game = false; }

    let draw = true;
    for (let i = 0; i < 7; i++) draw &= board.top[i] == -1;
    if (draw) { active_game = false; }

    if (!active_game) {
        let message = "";
        if (score == HUMAN) {
            human_score += 1;
            message = "You won!"
        }
        else if (score == COMPUTER) {
            bot_score += 1;
            message = `${capitalize(current_bot)} won!`
        }
        else if (draw) {
            bot_score += 0.5;
            human_score += 0.5;
            message = "Draw!"
        }

        $('#human-score').text(`${human_score}`);
        $('#bot-score').text(`${bot_score}`);

        setTimeout(() => { 
            $('#result-text').text(message);
            result_popup_open();
        }, 500);

        return true;
    }

    return false;
}

function new_game() {
    board = empty_board();
    active_game = true;
    player = HUMAN;

    $('#board>span').removeAttr("token_placed");
}

function change_bot_open() {
    $('#change-bot-dialog').show();
}

function change_bot_close() {
    $('#change-bot-dialog').hide();
}

function select_bot_click(bot) {
    current_bot = bot;

    $('#bot-icon').attr("src",`player_${bot}.png`);
    $('#bot-name').text(bot);
    mcts_iters = get_bot_difficulty(bot);

    change_bot_close();
}


function result_popup_open() {
    $('#result-popup').show();
}

function result_popup_close() {
    $('#result-popup').hide();
}


function capitalize(string) {
    return string.charAt(0).toUpperCase() + string.slice(1);
  }