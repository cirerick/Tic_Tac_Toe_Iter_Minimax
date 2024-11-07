mod turn;

/*

    fix player taking ai spaces 

*/


fn main() {

    println!("Welcome to Tic-Tac-Toe, please enter input in this format: x,y (1-3) or a single digit (1-9) to place X.");

    let mut board: Vec<Vec<char>> = vec![vec!['_'; 3],
                                         vec!['_'; 3],
                                         vec!['_'; 3]];

    print_board(&board);

    let mut terminal: i8 = 0;

    loop {
        match terminal {
            0 => {
                terminal = turn::player_turn(&mut board);
                print_board(&board);
            },
            1 => {
                println!("AI WINS!");
                return
            }
            _ => {
                println!("CAT...");
                return
            }
        }

        match terminal {
            0 => {
                terminal = turn::ai_turn(&mut board);
                print_board(&board);
            },
            1 => {
                println!("YOU WON!");
                return
            }
            _ => {
                println!("CAT...");
                return
            }
        }
    }

}

fn print_board(board: &Vec<Vec<char>>) {
    println!("========================");
    for _i in 0..board.len(){
        for _j in 0..board[_i].len(){
            print!("{} ", board[_i][_j]);
        }
        println!();
    }
    println!();
}