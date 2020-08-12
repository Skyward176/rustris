fn main() {
    let mut game = board_constructor(10,20);
    //draw_board(&game);
    game = spawn_i_block(game);
    draw_board(&game);
}
fn draw_board(game: &Board) {
    for i in 0..game.height {
        for x in 0..game.width {
            print!("{}", game.board[i][x]);
        }
        print!("\n");
    }
}
fn board_constructor(width: usize, height: usize) -> Board {
    let mut game = Board {
        width,
        height,
        board: vec![vec!["#".to_string();width]; height],
    };
    return game;
}
struct Board {
    board: Vec<Vec<String>>,
    width: usize,
    height: usize,    
}
fn spawn_i_block(mut board: Board) -> Board{
    let pos_x = 3;
    let pos_y = 0;
    let shape = [0; 4];
    println!("{:?}", shape);
    for i in 0..shape.len() {
        board.board[pos_y][pos_x] = shape[i].to_string();
    }
    board
}
