pub struct Game {
    board: [char; 9],
    players: [char; 2],
    cur_player_index: usize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Game::create_board(),
            players: ['X', 'O'],
            cur_player_index: 0,
        }
    }

    pub fn make_move(&mut self, pos: usize) -> Result<char, &'static str> {
        if pos > 8 {
            return Err("Not a valid position!");
        } else if self.board[pos] != ' ' {
            return Err("That place is taken!");
        }

        self.board[pos] = self.get_cur_player();
        self.next_turn();
        Ok(self.get_cur_player())
    }

    pub fn get_free_spaces(&self) -> Vec<usize> {
        self.board
            .iter()
            .enumerate()
            .filter_map(|(i, v)| if *v == ' ' { Some(i) } else { None })
            .collect()
    }

    pub fn check_filled(&self) -> bool {
        for cell in self.board.iter() {
            if cell == &' ' {
                return false;
            }
        }

        true
    }

    pub fn check_win(&self) -> char {
        // Rows
        for ri in (0..9).step_by(3) {
            let left = self.board[ri];
            let mid = self.board[ri + 1];
            let right = self.board[ri + 2];

            if Game::check_line(left, mid, right) {
                return left;
            }
        }

        // Columns
        for ri in 0..3 {
            let top = self.board[ri];
            let mid = self.board[ri + 3];
            let bot = self.board[ri + 6];

            if Game::check_line(top, mid, bot) {
                return top;
            }
        }

        // Diagonals
        let tl = self.board[0];
        let m = self.board[4];
        let br = self.board[8];

        if Game::check_line(tl, m, br) {
            return tl;
        }

        let tr = self.board[2];
        let bl = self.board[6];

        if Game::check_line(tr, m, bl) {
            return tr;
        }

        ' '
    }

    fn check_line(x: char, y: char, z: char) -> bool {
        x == y && x == z && x != ' '
    }

    fn create_board() -> [char; 9] {
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']
    }

    fn next_turn(&mut self) {
        self.cur_player_index += 1;
        self.cur_player_index %= 2;
    }

    pub fn get_cur_player(&self) -> char {
        self.players[self.cur_player_index]
    }

    pub fn get_board(&self) -> [char; 9] {
        self.board
    }

    pub fn display_board(&self) {
        println!("     |     |     ");
        println!(
            "  {}  |  {}  |  {}  ",
            self.board[6], self.board[7], self.board[8]
        );
        println!("    7|    8|    9");
        println!("-----+-----+-----");
        println!("     |     |     ");
        println!(
            "  {}  |  {}  |  {}  ",
            self.board[3], self.board[4], self.board[5]
        );
        println!("    4|    5|    6");
        println!("-----+-----+-----");
        println!("     |     |     ");
        println!(
            "  {}  |  {}  |  {}  ",
            self.board[0], self.board[1], self.board[2]
        );
        println!("    1|    2|    3");
    }
}
