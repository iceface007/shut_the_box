use rand::Rng;

struct TwoDice {
    first_dice: i8,
    second_dice: i8,
}

#[derive(Copy, Clone)]
struct Board {
    moves: i8,
    open_flaps: [bool; 9],
} 

impl Board {
    fn roll_the_dice(self: &mut Board) -> TwoDice {
        self.moves += 1;

        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let two_dice: TwoDice = TwoDice { 
            first_dice: rng.gen_range(1..6), 
            second_dice: rng.gen_range(1..6),
        };

        two_dice
    }

    fn close_flaps(self: &mut Board, number_of_points: i8) {

        let index: usize = (number_of_points - 1).try_into().unwrap();
        self.open_flaps[index] = true;
    }
}

impl TwoDice {
    fn sum_of_dice(self: &TwoDice) -> i8 {
        let sum: i8 = self.first_dice + self.second_dice;

        sum
    }
}
fn main() {
    let mut board: Board = Board { 
        moves: 0, 
        open_flaps: [false; 9], 
    };
    
    visualize(board.open_flaps);
    let dice: TwoDice = board.roll_the_dice();

    println!("First dice: {:?}", dice.first_dice);
    println!("Second dice: {:?}", dice.second_dice);

    let sum = dice.sum_of_dice();

    println!("Sum of dice: {sum}");
    board.close_flaps(dice.first_dice);

    visualize(board.open_flaps);
}

//Nächster Schritt: fn play_a_turn()

fn visualize(board: [bool; 9]) {
    for i in 0..9 {
        if board[i] {
            print!("| x ");
            if i >= 9 {
                print!(" ");
            }
        } else {
            print!("|   ");
            if i >= 9 {
                print!(" ");
            }
        }
    }

    println!("|");

    for i in 0..9 {
        print!("| {} ", i + 1);
    }
    println!("|");
}