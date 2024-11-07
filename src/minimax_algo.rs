/*

implement memory allocation for state

let it be a method

pointer_alloc(State) -> mut* State{
    //do allocating haha funny stuff
}


*/



#[derive(Debug, Eq, PartialEq)]
struct State {
    board: Vec<Vec<char>>,
    branches: Vec<*mut State>, 
    score: Option<i8>,
    depth: usize, //tree levels shows who's turn it is to choose //min is even levels, max are odd levels
    moved: (usize, usize), 
    // best: i32, //dependent on who's move, if min turns -> set to infinity, if max turn -> set to -infinity
    // alpha: i32, //max manipulates this -> set to -infinity
    // beta: i32, //min manipulates this -> set to infinity 
}

// Custom Ord and PartialOrd implementations for sorting by score
// impl Ord for State {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         match (self.score, other.score) {
//             (Some(s1), Some(s2)) => s1.cmp(&s2), // Compare Some values directly in ascending order
//             (None, Some(_)) => std::cmp::Ordering::Less,   // Treat None as less than any Some value
//             (Some(_), None) => std::cmp::Ordering::Greater,
//             (None, None) => std::cmp::Ordering::Equal,
//         }
//     }
// }

// impl PartialOrd for State {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }


//AI 'O' Mini
//Player 'X' Max
pub fn minimax(board: &Vec<Vec<char>>) -> (usize, usize){
    //prototyping
    #[allow(unused_assignments)]
    let mut best_move: (usize, usize) = (0, 0); //get this at lv 1


    //get stack
    let mut stack: Vec<*mut State> = Vec::new();

    //root
    
    stack.push(allocate_state(board.clone(), vec![], None, 0, (0, 0)));
    

    let max_char: char = 'X'; //Player char
    let min_char: char = 'O'; //AI char
    
    let mut found_none: bool = true;
    let mut elem: usize = 0; // our tracker
    let mut depth: usize = 1;
    /*
    
        we want to keep track of how many times a player can go.
        then we start 
        we alternating, simulating turn changes

    */
    while found_none { //needs to be reworked
        found_none = false; //assume
        //max turn, push all spots it could possibly go on
        /* keep track of all nodes that have been changed */
        let mut stack_size: usize = stack.len();
        while elem < stack_size {
            unsafe{
                if evaluate(&(*stack[elem]).board, max_char).is_none() {
                    for _min_i in 0..3{
                        for _min_j in 0..3{
                            if (*stack[elem]).board[_min_i][_min_j] == '_' {
                                let mut temp_board: Vec<Vec<char>> = (*stack[elem]).board.clone();
                                temp_board[_min_i][_min_j] = min_char;
                                let new_ptr: *mut State = allocate_state(temp_board.clone(), vec![], evaluate(&temp_board, min_char).map(|s| (-s)), depth, (_min_i, _min_j));
                                stack.push(new_ptr);
                                (*stack[elem]).branches.push(new_ptr);
                            }
                        }
                        found_none = true;
                    }//end of finding empty space for min
                }
                elem += 1;
            }
        }

        depth += 1;
        stack_size = stack.len();

        //max turn to populate it's level   
        while elem < stack_size {
            unsafe {
                if evaluate(&(*stack[elem]).board, min_char).is_none() {
                    for _max_i in 0..3{
                        for _max_j in 0..3{
                            if (*stack[elem]).board[_max_i][_max_j]  == '_' {
                                let mut temp_board: Vec<Vec<char>> = (*stack[elem]).board.clone();
                                temp_board[_max_i][_max_j] = max_char;
                                // println!("score:{:?} for: {}{}, depth: {}", evaluate(&temp_board, max_char), _max_i, _max_j, depth);
                                let new_ptr: *mut State = allocate_state(temp_board.clone(), vec![], evaluate(&temp_board, max_char), depth, (_max_i, _max_j)); //allocate new node
                                stack.push(new_ptr);
                                (*stack[elem]).branches.push(new_ptr);
                                //checking for none to continue building tree
                            }
                        }
                    }//end of finding empty space for max
                    found_none = true;
                }
                elem += 1;
            }    
        }

        depth += 1;
    }

    let mut temp_stack: Vec<*mut State> = stack.clone(); //we will be using this to deallocate everything after we get the best move

    //ripple back the stack until root is left to find the best move
    let mut max_turn: bool;

    let mut best_score: i8;
    loop {
        // println!("Stack Len: {}", stack.len());
        let ptr_state: *mut State = stack.pop().unwrap();

        //check if there are any branches
        if unsafe{(*ptr_state).branches.is_empty()} {
            continue
        }

        if (unsafe{(*ptr_state).depth} + 2) % 2 == 0 {
            max_turn = false;
        } else {
            max_turn = true;
        }

        unsafe{merge_sort(&mut (*ptr_state).branches);}
        // unsafe{(*ptr_state).branches.sort_by(|a, b| a.cmp(b));}

        let i: usize = unsafe{(*ptr_state).branches.len() - 1};

        if max_turn {
            //pick the max score from the ptr vec
            match unsafe{(*(*ptr_state).branches[i]).score} {
                Some(s) => best_score = s,
                None => best_score = 0,
            }
            unsafe {(*ptr_state).score = Some(best_score)};
        } else {
            //pick the min from the ptr vec
            if stack.is_empty() {
                
                // unsafe {print_states(&(*ptr_state).branches)};
                best_move = unsafe{(*(*ptr_state).branches[0]).moved};
                //deallocate everything
                // println!("{:?}", best_move); 
                unsafe {deallocate_states(&mut temp_stack)}; 
                return best_move
            }
            // print_states(&stack);
            match unsafe{(*(*ptr_state).branches[0]).score} {
                Some(s) => best_score = s,
                None => best_score = 0,
            }
            unsafe {(*ptr_state).score = Some(best_score);}
        }
    }
       
    // return best_move
}

/* 
fn print_states(states: &Vec<*mut State>) {
    for hey in states {
       unsafe{ println!("{:?}", **hey);}
    }
}
*/

//evaluate if the board is terminal
fn evaluate(board: &Vec<Vec<char>>, t: char) -> Option<i8>{
    /*
         0 cat 
         None non terminal
         1 ai/player won
     */

    //horizontal check 
    for _i in 0..board.len(){
        for _j in 0..board[_i].len(){
            if board[_i][_j] == t {
                if _j == (board[_i].len() - 1){
                    return Some(1)
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
                    return Some(1)
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
                return Some(1);
            }
        } else {
            break;
        }
    }

    //top-right to bottom-left
    for _i in 0..board.len(){
        if board[_i][board.len() - 1 -_i] == t{
            if _i == board.len() - 1{
                return Some(1)
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
                    return Some(0)
                }
            } else {
                return None
            }
        }
    }

    return None
}

fn merge_sort(vec: &mut Vec<*mut State>) {
    
    //helper merge function to merge dependent on indices
    fn merge(m_vec: &mut Vec<*mut State>, m_temp: &mut Vec<*mut State>, left: usize, mid: usize, right: usize) {
        let mut i = left;
        let mut j = mid;
        let mut k = left;
        
        while i < mid && j < right {
            if unsafe{(*m_vec[i]).score <= (*m_vec[j]).score} {
                m_temp[k] = m_vec[i];
                i += 1;
            } else {
                m_temp[k] = m_vec[j];
                j += 1;
            }
            k += 1;
        }
        
        // Copy remaining elements
        while i < mid {
            m_temp[k] = m_vec[i];
            i += 1;
            k += 1;
        }
        
        while j < right {
            m_temp[k] = m_vec[j];
            j += 1;
            k += 1;
        }
    }
    
    let n: usize = vec.len(); //get len of vec
    let mut temp: Vec<*mut State> = vec.clone(); //temp vec for merging
    
    let mut size: usize = 1;
    
    while size < n {
        let mut left: usize = 0;
        
        while left < n {
            let mid: usize = left + size;
            let right: usize = if (left + 2 * size) >= n {
                n
            } else {
                left + 2 * size
            };                      
            
            if mid < right {
                merge(vec, &mut temp, left, mid, right);
            }
            
            left += 2 * size;
        }
        
        //update changes to real vec
        *vec = temp.clone();
        
        size *= 2;
    }
}


fn allocate_state(board: Vec<Vec<char>>, ptr_vec: Vec<*mut State> ,score: Option<i8>, depth: usize, moved: (usize, usize)) -> *mut State { //allocate state and return a pointer to allocated address
    unsafe {
        let layout: std::alloc::Layout = std::alloc::Layout::new::<State>();
        let ptr: *mut State = std::alloc::alloc(layout) as *mut State;
        if ptr.is_null() {std::alloc::handle_alloc_error(layout);}
        //set values of passed State to allocated State
        ptr.write(State {
            board: board,
            branches: ptr_vec,
            score: score,
            depth: depth,
            moved: moved
        });
        return ptr
    }
}

unsafe fn deallocate_states(states: &mut Vec<*mut State>){
    while !states.is_empty(){
        let state_ptr: *mut State = states.pop().unwrap();
        let layout: std::alloc::Layout = std::alloc::Layout::new::<State>();
        std::ptr::drop_in_place(state_ptr);
        std::alloc::dealloc(state_ptr as *mut u8, layout);
    }
}