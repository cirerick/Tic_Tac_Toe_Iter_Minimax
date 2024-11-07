#[path = "lib/cin.rs"]
mod cin;

//return bool later
pub fn player_turn(board: &mut Vec<Vec<char>>) -> i8{

    //helper function to validate input
    fn valid_input(input: &mut String, board: &Vec<Vec<char>>) -> bool {
        if input.len() == 0 {
            return false;
        }


        /*
            will allow, with range of 1-9: 
            9,9 = len 3
            9, 9 = len 4
            (9,9) = len 5
            (9, 9) = len 6   
        */
        //

        //convert strings to bytes
        let mut _s_to_bytes: Vec<u8> = input.as_bytes().to_vec();
        
        //remove the () if they are present
        if _s_to_bytes[0] == 40 && _s_to_bytes[_s_to_bytes.len() - 1] == 41{
            _s_to_bytes.remove(0);
            _s_to_bytes.remove(_s_to_bytes.len() - 1);
        }

        //remove white spaces if any
        _s_to_bytes.retain(|&x| x != 32);

        //single digit validity
        if _s_to_bytes.len() == 1 {
            if _s_to_bytes[0] < 49 || _s_to_bytes [0] > 57 {
                return false;
            } else if board[((_s_to_bytes[0] - 49) / 3) as usize][((_s_to_bytes[0] - 49) % 3) as usize] == '_' {
                *input = String::from_utf8(_s_to_bytes).unwrap();
                return true
            } else {
                print!("Space taken. ");
                return false
            }
        }

        //make sure size is 3
        if _s_to_bytes.len() != 3 {
            return false
        }
        
        //see if within range of valid input
        // println!("{:?}", _s_to_bytes);
        for _i in 0.._s_to_bytes.len() {
            if (_i + 2) % 2 == 0 {
                if _s_to_bytes[_i] < 49 || _s_to_bytes[_i] > 51 {
                    return false;
            }
            } else {
                if _s_to_bytes[_i] != 44 {
                    return false;
                }
            }
        }

        //check if space is taken
        if board[(_s_to_bytes[0] - 49) as usize][(_s_to_bytes[2] - 49) as usize] != '_' {
            print!("Space taken. ");
            return false
        }
        
        *input = String::from_utf8(_s_to_bytes).unwrap();
        return true
        
    }    
    
    
    
    print!("Enter input: ");
    let mut player_input: String = cin::cin_string(); 
    // println!("Value recived");

    while !valid_input(&mut player_input, &board) {
        // println!("{:?}", player_input.as_bytes());
        print!("Invalid response, enter it again: ");
        player_input = cin::cin_string();
    }

    let s_to_b: &[u8] = player_input.as_bytes();

    let (x, y): (usize, usize) = match s_to_b.len() {
        1 => {(((s_to_b[0] - 49) / 3) as usize, ((s_to_b[0] - 49) % 3) as usize)},
        _ => {((s_to_b[0] - 49) as usize, (s_to_b[2] - 49) as usize)},
    };

    // println!("You chose: ({}, {})", x, y);
    board[x][y] = 'X'; 

    return terminal(&board, 'X');

}


#[path = "minimax_algo.rs"]
mod minimax_algo;


//ai's turn
pub fn ai_turn(board: &mut Vec<Vec<char>>) -> i8{
    let (x, y): (usize, usize) = minimax_algo::minimax(&board);

    board[x][y] = 'O';

    return terminal(board, 'O');
}

/*
    OOO (0,0) (0,1) (0,2)
    XXX (1,0) (1,1) (1,2)
    OOO (2,0) (2,1) (2,2)

    OXO 
    OXO
    OXO

    OXO (0,0) (1,1) (2,2)
    XOX (0,2) (1,1) (2,0)
    OXO

*/

fn terminal(board: &Vec<Vec<char>>, t: char) -> i8{
    /*
        -1 cat 
         0 non terminal
         1 ai/player won
     */

    //horizontal check 
    for _i in 0..board.len(){
        for _j in 0..board[_i].len(){
            if board[_i][_j] == t {
                if _j == (board[_i].len() - 1){
                    return 1
                }
            } else {
                break;
            }
        }
    }

    //vertical check
    for _j in 0..board.len(){
        for _i in 0..board.len() {
            if board[_i][_j] == t{
                if _i == board.len() - 1 {
                    return 1
                }
            } else {
                break;
            }
        }
    } 

    //diagnol check top-left to bottom-right
    for _i in 0..board.len(){
        if board[_i][_i] == t {
            if _i == board.len() - 1 {
                return 1;
            }
        } else {
            break;
        }
    }

    //top-right to bottom-left
    for _i in 0..board.len(){
        if board[_i][board.len() - 1 -_i] == t{
            if _i == board.len() - 1{
                return 1
            }
        } else {
            break;
        }
    }

    //check if board is full
    for _i in 0..board.len(){
        for _j in 0..board[_i].len(){
            if board[_i][_j] == 'X' || board[_i][_j] == 'O'{
                if _i == board.len() - 1 && _j == board.len() - 1{
                    return -1
                }
            } else {
                return 0
            }
        }
    }
    


    return 0
}