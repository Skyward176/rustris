fn main() {
    let game = board_constructor(10,20);
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