use std::{fmt};
use super::mcts::GeneralGame;

#[derive(Debug, Clone, PartialEq)]
pub struct Connect4<const ROWS:usize,const COLUMNS:usize,const CONNECT:usize>{
    pub board: [[i8;COLUMNS];ROWS] // (0,0) is in the top-left corner, indexing is (row,column)
}

impl<const ROWS:usize,const COLUMNS:usize,const CONNECT:usize> Connect4<ROWS,COLUMNS,CONNECT> {    
    pub fn empty() -> Connect4<ROWS,COLUMNS,CONNECT> {
        return Connect4 {board:[[0;COLUMNS];ROWS]};
    }

    /// takes in a string where each line contains 'X', 'O', '.' and ends with '\n' or '\r'
    pub fn from_string(val : &str) -> Option<Connect4<ROWS,COLUMNS,CONNECT>> {
        let mut connect4 = Connect4::<ROWS,COLUMNS,CONNECT>::empty();

        let mut row = 0usize;
        let mut column = 0usize;

        for s in val.chars(){
            if s == '\n' || s == '\r' {
                if column != COLUMNS { return None }

                row += 1;
                column = 0;

                continue;
            }
            if row >= ROWS { return None }
            if column >= COLUMNS { return None }

            let val = match s {
                '.' => 0,
                'X' => 1,
                'O' => -1,
                _ => return None
            };
            connect4.board[row][column] = val;

            column += 1;
        }
        // check for gaps
        for col in 0..COLUMNS{
            let mut gap = false;
            for row in (0..ROWS).rev() {
                if connect4.board[row][col] == 0 {
                    gap = true;
                }
                if connect4.board[row][col] != 0 && gap {
                    return None;
                }
            }
        }

        return Some(connect4)
    }
}

impl<const ROWS:usize,const COLUMNS:usize,const CONNECT:usize> fmt::Display for Connect4<ROWS,COLUMNS,CONNECT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..ROWS{
            for col in 0..COLUMNS{
                write!(f, "{} ", if self.board[row][col] == 1 {'X'} else if self.board[row][col] == -1 {'O'} else {'.'}).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        write!(f, "")
    }
}

impl<const ROWS:usize,const COLUMNS:usize,const CONNECT:usize> GeneralGame for Connect4<ROWS,COLUMNS,CONNECT> {
    fn get_score(&self) -> i8 {
        let mut count_1 : usize;
        let mut count_n1 : usize;

        // Rows
        for row in 0..ROWS {
            count_1 = 0;
            count_n1 = 0;
            for col in 0..COLUMNS {
                if self.board[row][col] == -1 { count_n1 += 1; }
                else {count_n1 = 0; }

                if self.board[row][col] == 1 { count_1 += 1; }
                else {count_1 = 0; }

                if count_1 >= CONNECT {return 1;}
                if count_n1 >= CONNECT {return -1;}
            }
        }

        // Columns
        for col in 0..COLUMNS {
            count_1 = 0;
            count_n1 = 0;
            for row in 0..ROWS {
                if self.board[row][col] == -1 { count_n1 += 1; }
                else {count_n1 = 0; }

                if self.board[row][col] == 1 { count_1 += 1; }
                else {count_1 = 0; }

                if count_1 >= CONNECT {return 1;}
                if count_n1 >= CONNECT {return -1;}
            }
        }

        let min_offset:i32 = (CONNECT as i32)-(ROWS as i32);
        let max_offset:i32 = (COLUMNS as i32)-(CONNECT as i32);
        // diagonal in this / direction
        for offset in min_offset..=max_offset {
            count_1 = 0;
            count_n1 = 0;
            for col in offset.max(0)..((ROWS as i32) + offset).min(COLUMNS as i32) {
                let row = (ROWS as i32) - 1 + offset - col;

                if self.board[row as usize][col as usize] == -1 { count_n1 += 1; }
                else {count_n1 = 0; }

                if self.board[row as usize][col as usize] == 1 { count_1 += 1; }
                else {count_1 = 0; }

                if count_1 >= CONNECT {return 1;}
                if count_n1 >= CONNECT {return -1;}
            }
        }

        // diagonal in this \ direction
        for offset in min_offset..=max_offset {
            count_1 = 0;
            count_n1 = 0;
            for col in offset.max(0)..((ROWS as i32) + offset).min(COLUMNS as i32) {
                let row = col - offset;

                if self.board[row as usize][col as usize] == -1 { count_n1 += 1; }
                else {count_n1 = 0; }

                if self.board[row as usize][col as usize] == 1 { count_1 += 1; }
                else {count_1 = 0; }

                if count_1 >= CONNECT {return 1;}
                if count_n1 >= CONNECT {return -1;}
            }
        }

        return 0;
    }

    fn get_available(&self) -> Vec<usize> {
        return Vec::from_iter( (0..COLUMNS).filter(|&col| self.board[0][col] == 0) );
    }

    fn update(&mut self, index:usize, player:i8) {
        for row in (0..ROWS).rev() {
            if self.board[row][index] == 0 {
                self.board[row][index] = player;
                return;
            }
        }

        panic!("Out of range.");
    }
}

#[test]
fn test_connect4_fromstr() {
    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str), Some(Connect4 {board: [[1,0,0,0,0,0],[-1,0,0,0,0,0],[1,0,0,-1,0,0],[-1,0,0,1,0,0],[1,-1,0,1,1,0],[1,1,0,-1,-1,-1]]}));

    let str = "\
                        X..O..\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str), None);

    let str = "\
                        X..O..\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str), None);

    let str = "\
                        X.XO...\n\
                        O.OX...\n\
                        XOOOX..\n\
                        OXXXOO.\n\
                        XOOXXO.\n\
                        XOXOXO.\n\
                    ";
    assert_eq!(Connect4::<6,7,4>::from_string(str), Some(Connect4 {board: [[1,0,1,-1,0,0,0],[-1,0,-1,1,0,0,0],[1,-1,-1,-1,1,0,0],[-1,1,1,1,-1,-1,0],[1,-1,-1,1,1,-1,0],[1,-1,1,-1,1,-1,0]]}));
}

#[test]
fn test_connect4_score() {
    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        OOOOOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_score(), -1);

    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_score(), 0);

    let str = "\
                        X.....\n\
                        X.....\n\
                        X..O..\n\
                        X..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_score(), 1, "vertical");

    let str = "\
                        X.X...\n\
                        O.XX..\n\
                        X.XOX.\n\
                        O.OXOX\n\
                        XOOXXO\n\
                        OOXOXO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_score(), -1, "diagonal y=x");

    let str = "\
                        X.XO...\n\
                        O.OX...\n\
                        XOOOX..\n\
                        OXXXOO.\n\
                        XOOXXO.\n\
                        XOXOXO.\n\
                    ";
    assert_eq!(Connect4::<6,7,4>::from_string(str).unwrap().get_score(), -1, "diagonal y=x");

    let str = "\
                        X.XO...\n\
                        O.XX...\n\
                        XOOOX.O\n\
                        OXXXOOX\n\
                        XOOXOOX\n\
                        XOXOXOX\n\
                    ";
    assert_eq!(Connect4::<6,7,4>::from_string(str).unwrap().get_score(), -1, "diagonal y=x");

    let str = "\
                        X.....\n\
                        X.....\n\
                        O..O..\n\
                        XO.X..\n\
                        XOOXX.\n\
                        XXOOOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_score(), -1, "diagonal y=-x");

    let str = "\
                        X..O..\n\
                        O.OO..\n\
                        XOXO..\n\
                        OXXX..\n\
                        XOOXX.\n\
                        OOOXOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_score(), -1);

    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        OX.X..\n\
                        XOXXX.\n\
                        OOOXOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_score(), 1);

    let str = "\
                        O.X...\n\
                        X.XX..\n\
                        OXOXX.\n\
                        XOXOXX\n\
                        OOOXOO\n\
                    ";
    assert_eq!(Connect4::<5,6,4>::from_string(str).unwrap().get_score(), 1);
}

#[test]
fn test_connect4_available() {
    let str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        OOOOOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_available(), [1,2,3,4,5]);

    let str = "\
                        X..OX.\n\
                        O..XO.\n\
                        X..OX.\n\
                        O..XX.\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_available(), [1,2,5]);

    let str = "\
                        ......\n\
                        X.....\n\
                        X..O..\n\
                        X..X..\n\
                        XO.XX.\n\
                        XX.OOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_available(), [0,1,2,3,4,5]);

    let str = "\
                        XXOXXO\n\
                        XOOOOO\n\
                        XOOOOO\n\
                        XOOXOO\n\
                        XOXXXX\n\
                        XXXOOO\n\
                    ";
    assert_eq!(Connect4::<6,6,4>::from_string(str).unwrap().get_available(), []);

    let str = "\
                        O..XO..\n\
                        X..OX..\n\
                        O..XX..\n\
                        XO.XX..\n\
                        XX.OOO.\n\
                    ";
    assert_eq!(Connect4::<5,7,4>::from_string(str).unwrap().get_available(), [1,2,5,6]);
}

#[test]
fn test_connect4_fmt(){
    let connect4 = Connect4::<6,6,4>{ board: [[-1,0,0,1,0,0],[0,0,0,0,0,0],[0,0,0,0,0,0],[0,0,0,0,0,0],[0,0,1,0,-1,1],[-1,0,1,0,0,-1]]};

    let connect4_str = format!("{}", connect4);
    assert_eq!(connect4_str, "O . . X . . \n. . . . . . \n. . . . . . \n. . . . . . \n. . X . O X \nO . X . . O \n");

    let connect4 = Connect4::<3,6,4>{ board: [[0,0,0,0,0,0],[0,0,1,0,-1,1],[-1,0,1,0,0,-1]]};

    let connect4_str = format!("{}", connect4);
    assert_eq!(connect4_str, ". . . . . . \n. . X . O X \nO . X . . O \n");
}

#[test]
fn test_connect4_update(){
    let mut str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        OOOOO.\n\
                    ";
    let connect4 = Connect4::<6,6,4>::from_string(str).unwrap();

    str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        OX.X..\n\
                        XO.XX.\n\
                        OOOOO.\n\
                    ";
    let mut test = connect4.clone();
    test.update(1, 1);
    assert_eq!(test, Connect4::from_string(str).unwrap());

    str = "\
                        X.....\n\
                        O.....\n\
                        X..O..\n\
                        O..X..\n\
                        XO.XX.\n\
                        OOOOOO\n\
                    ";
    test = connect4.clone();
    test.update(5, -1);
    assert_eq!(test, Connect4::from_string(str).unwrap());

    str = "\
                O......\n\
                X..O...\n\
                O..X..X\n\
                XO.XX.O\n\
                OOOOOOX\n\
            ";
    let mut test = Connect4::<5,7,4>::from_string(&str).unwrap();
    test.update(6, -1);
    str = "\
                O......\n\
                X..O..O\n\
                O..X..X\n\
                XO.XX.O\n\
                OOOOOOX\n\
            ";
    assert_eq!(test, Connect4::from_string(str).unwrap());
}