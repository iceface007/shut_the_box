use rand::Rng;

struct TwoDice {
    first_dice: i8,
    second_dice: i8,
}
struct Board {
    moves: i8,
    open_flaps: [bool; 9],
    //Strategie/Heuristik kann man dann später hier noch irgendwo implementiern 
} 

impl Board {
    fn roll_the_dice(mut self: Board) -> TwoDice {
        self.moves += 1;

        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let two_dice: TwoDice = TwoDice { 
            first_dice: rng.gen_range(1..6), 
            second_dice: rng.gen_range(1..6),
        };

        two_dice
    }
}

impl TwoDice {
    fn sum_of_dice(self: &TwoDice) -> i8 {
        let sum: i8 = self.first_dice + self.second_dice;

        sum
    }

    //Auch ein möglicher nächster Schritt: fn close_flap()
}
fn main() {
    let board: Board = Board { 
        moves: 0, 
        open_flaps: [false; 9], 
    };
    
    visualize(board.open_flaps);
    let dice: TwoDice = board.roll_the_dice();

    println!("First dice: {:?}", dice.first_dice);
    println!("Second dice: {:?}", dice.second_dice);

    let sum = dice.sum_of_dice();

    print!("Sum of dice: {sum}");
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