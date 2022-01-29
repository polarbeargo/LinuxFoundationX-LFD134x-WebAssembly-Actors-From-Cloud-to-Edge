#[derive(Debug, Clone, PartialEq, Copy)]
enum GamePiece {
    X = 2,
    O = 1,
    Empty = 0,
}

static mut GAME_BOARD: &'static mut [GamePiece] = &mut [GamePiece::Empty; 9];
static mut TURN: GamePiece = GamePiece::X;

fn get_piece(index: usize) -> GamePiece {
    unsafe { GAME_BOARD[index] }
}

fn set_piece(index: usize, piece: GamePiece) {
    unsafe { GAME_BOARD[index] = piece }
}

fn advance_turn() {
    unsafe {
        if TURN == GamePiece::X {
            TURN = GamePiece::O;
        } else {
            TURN = GamePiece::X;
        }
    }
}

fn index_for_position(row: i32, col: i32) -> usize {
    (row * 3 + col) as usize
}

#[no_mangle]
#[export_name = "initGame"]
pub extern "C" fn init_game() {
    for row in 0..3 {
        for col in 0..3 {
            set_piece(index_for_position(row, col), GamePiece::Empty);
        }
    }
    unsafe {
        TURN = GamePiece::X;
    }
}

#[no_mangle]
#[export_name = "takeTurn"]
pub extern "C" fn take_turn(row: i32, col: i32) {
    set_piece(index_for_position(row, col), unsafe { TURN });
    advance_turn();
}

#[no_mangle]
#[export_name = "currentTurn"]
pub extern "C" fn current_turn() -> i32 {
    unsafe { TURN as i32 }
}

#[no_mangle]
#[export_name = "getPiece"]
pub extern "C" fn pub_get_piece(row: i32, col: i32) -> i32 {
    get_piece(index_for_position(row, col)) as i32
}
