/*
Copyright Robert Webb

(for now)

--------

Written one late Monday night
*/

use std::io;
use std::fmt;


#[derive(Clone, Copy)]
enum Cell {
    X,
    O,
    Empty
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
            Cell::Empty => write!(f, "-"),
        }
    }
}


#[derive(PartialEq)]
enum Player {
    Crosses,
    Noughts,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Player::Crosses => write!(f, "Crosses"),
            Player::Noughts => write!(f, "Noughts"),
        }
    }
}

fn cell_pos_from_id(cell: usize) -> (usize, usize) {
    (cell.checked_div(3).unwrap(), cell % 3)
}

// TODO: Make the grid into a class/struct
fn print_grid(grid: [Cell; 9]) {
    println!("{}{}{}", grid[0], grid[1], grid[2]);
    println!("{}{}{}", grid[3], grid[4], grid[5]);
    println!("{}{}{}", grid[6], grid[7], grid[8]);
}

fn check_triple(c1: Cell, c2: Cell, c3: Cell) -> Option<Player> {
    match (c1, c2, c3) {
        (Cell::O, Cell::O, Cell::O) => Some(Player::Noughts),
        (Cell::X, Cell::X, Cell::X) => Some(Player::Crosses),
        (_, _, _) => None,
    }
}

fn check_winner(grid: [Cell; 9]) -> Option<Player> {
    // all of the winning combinations
    let win_triples = vec![
        // horizontal
        (0, 1, 2),
        (3, 4, 5),
        (6, 7, 8),
        // vertical
        (0, 3, 6),
        (1, 4, 7),
        (2, 5, 8),
        // diagonal
        (0, 4, 8),
        (2, 4, 6),
    ];

    win_triples
        .into_iter()
        .filter_map(|(c1, c2, c3)| {
            check_triple(grid[c1], grid[c2], grid[c3])
        })
        .next()
}

fn main() {
    println!("Welcome to the game of TIC TAC TOE");

    // TODO: make the grid into an object/class/struct
    let mut grid: [Cell; 9] = [Cell::Empty; 9];

    let mut current_player = Player::Noughts;
    
    loop {
        let mut cell = String::new();

        println!("");
        println!("-------------");
        println!("It is {}'s turn", current_player);
        println!("");

        // display the grid
        print_grid(grid);
        
        println!("Please enter a number between 1 and 9");
        
        io::stdin().read_line(&mut cell)
            .expect("Failed to read line");

        // parse user input
        let cell: usize = match cell.trim().parse() {
            // valid value
            Ok(num) if (num >= 1 && num <= 9) => num,

            // an integer, but outside of the range
            Ok(bad_num) => {
                println!("The value must be between 1 and 9! ({} was given)", bad_num);
                continue;
            },

            // not an integer or valid
            Err(_) => {
                println!("Invalid value {} was given! It must be an integer between 1 and 9", cell);
                continue;
            },
        };
        
        println!("User gave the value {}", cell);

        // internally we count cell numbers from 0 to 8,
        // not 1 to 9
        let i = cell - 1;

        // get the cell coordinates for those watching at home
        let (y, x) = cell_pos_from_id(i);
        println!("User selected cell {}, {}", y, x);

        // depending on the current player, place whichever symbol
        grid[i] = match current_player {
            Player::Noughts => Cell::O,
            Player::Crosses => Cell::X,
        };

        match check_winner(grid) {
            Some(player) => {
                println!("{} has won!", player);
                break;
            },
            None => (),
        }

        // swap player!
        current_player = match current_player {
            Player::Noughts => Player::Crosses,
            Player::Crosses => Player::Noughts,
        };
    }
    
    print_grid(grid);
}
