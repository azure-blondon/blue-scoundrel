use std::fmt::Display;

use rand::{rng, seq::SliceRandom};


#[derive(Clone, Copy)]
enum Color {
    Heart,
    Spade,
    Diamond,
    Club,
}

#[derive(Clone, Copy, PartialEq)]
enum Rank {
    Two = 2,
    Three,
    Ace,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Four,
}

impl Rank {
    fn value(&self) -> u32 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

#[derive(Clone)]
struct Card {
    pub color: Color,
    pub rank: Rank,
}

impl Card {
    fn new(color: Color, rank: Rank) -> Self {
        Card { color, rank }
    }

    fn deck() -> Vec<Card> {
        let mut deck = Vec::new();
        let colors = [Color::Heart, Color::Spade, Color::Diamond, Color::Club];
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Ace,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Four,
        ];

        for &color in &colors {
            for &rank in &ranks {
                deck.push(Card::new(color, rank));
            }
        }

        deck
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_str = match self.color {
            Color::Heart => "♥",
            Color::Spade => "♠",
            Color::Diamond => "♦",
            Color::Club => "♣",
        };

        let rank_str = match self.rank {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };

        write!(f, "{}{}", rank_str, color_str)
    }
}

struct Board {
    pub draw_pile: Vec<Card>,
    pub table_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub player_hand: Vec<Card>,
    pub player_hp: u32,
}


impl Board {
    fn new() -> Self {
        Board {
            draw_pile: Vec::new(),
            table_pile: Vec::new(),
            discard_pile: Vec::new(),
            player_hand: Vec::new(),
            player_hp: 20,
        }
    }

    fn setup(&mut self) {
        self.draw_pile = Card::deck();
        self.draw_pile.shuffle(&mut rand::rng());
    }

    fn show(&self) {
        println!("left: {}", self.draw_pile.len());
        println!(
            "room: {}",
            self.table_pile.iter().map(|card| card.to_string()).collect::<Vec<_>>().join(" ")
        );
        println!(
            "{:.2}hp  {}",
            self.player_hp,
            self.player_hand.iter().map(|card| card.to_string()).collect::<Vec<_>>().join(" ")
        );
    }

    fn filter_draw_pile(&mut self, condition: impl Fn(&Card) -> bool) {
        self.draw_pile.retain(|card| !condition(card));
    }

    fn draw_card(&mut self) -> Option<Card> {
        self.draw_pile.pop()
    }

    fn fill_room(&mut self) {
        while self.table_pile.len() < 4 {
            if let Some(card) = self.draw_card() {
                self.table_pile.push(card);
            } else {
                break;
            }
        }
    }

    fn discard_player_hand(&mut self) {
        self.discard_pile.append(&mut self.player_hand);
        self.player_hand.clear();
    }

    fn equip_weapon(&mut self, card: Card) {
        self.discard_player_hand();
        self.player_hand.push(card);
    }

    fn attack_with_weapon(&mut self, index: usize) {
        if let Some(weapon) = self.player_hand.get(0).cloned() {
            if let Some(target) = self.table_pile.get(index).cloned() {
                self.player_hand.push(self.table_pile.remove(index));
                let damage = target.rank.value().saturating_sub(weapon.rank.value());
                self.player_hp = self.player_hp.saturating_sub(damage);

            }
        }
    }

    fn attack_no_weapon(&mut self, index: usize) {
        if let Some(target) = self.table_pile.get(index).cloned() {
            self.discard_pile.push(self.table_pile.remove(index));
            let damage = target.rank.value();
            self.player_hp = self.player_hp.saturating_sub(damage);
        }
    }
}


fn handle_input(board: &mut Board) -> bool {
    use std::io::{self, Write};

    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    match parts.as_slice() {
        ["e", index_str] => {
            if let Ok(index) = index_str.parse::<usize>() {
                if index < board.table_pile.len() {
                    let card = board.table_pile.remove(index);
                    board.equip_weapon(card);
                } else {
                    println!("Invalid index.");
                }
            }
        }
        ["a", index_str] => {
            if let Ok(index) = index_str.parse::<usize>() {
                if index < board.table_pile.len() {
                    board.attack_no_weapon(index);
                } else {
                    println!("Invalid index.");
                }
            }
        }
        ["w", index_str] => {
            if let Ok(index) = index_str.parse::<usize>() {
                if index < board.table_pile.len() {
                    board.attack_with_weapon(index);
                } else {
                    println!("Invalid index.");
                }
            }
        }
        ["f"] => {
            board.fill_room();
        }
        ["r"] => {
            board.table_pile.shuffle(&mut rng());
            for _ in 0..=3 {
                if let Some(card) = board.table_pile.pop() {
                    board.draw_pile.insert(0, card);
                }
            }
            board.fill_room();
        }
        ["h", index_str] => { // heal
            if let Ok(index) = index_str.parse::<usize>() {
                if index < board.table_pile.len() {
                    let card = board.table_pile.remove(index);
                    board.player_hp += card.rank.value();
                    board.discard_pile.push(card);
                } else {
                    println!("Invalid index.");
                }
            }
        }
        ["d", index_str] => { // discard
            if let Ok(index) = index_str.parse::<usize>() {
                if index < board.table_pile.len() {
                    let card = board.table_pile.remove(index);
                    board.discard_pile.push(card);
                } else {
                    println!("Invalid index.");
                }
            }
        }
        ["d"] => {
            board.discard_player_hand();
        }
        ["q"] => return false,
        ["h"] => {
            println!("Commands:");
            println!("h - Show this help message");
            println!("e <index> - Equip weapon from table pile at index");
            println!("a <index> - Attack without weapon at table pile index");
            println!("w <index> - Attack with equipped weapon at table pile index");
            println!("r - Run from the room puts it at the back of the draw pile");
            println!("h <index> - Heal using card at table pile index");
            println!("d <index> - Discard card from table pile at index");
            println!("d - Discard your entire hand");
            println!("f - Fill the room with cards from draw pile");
            println!("q - Quit the game");
        }
        _ => println!("Invalid command."),
    }

    true
}


fn main() {
    let mut board = Board::new();
    board.setup();

    board.filter_draw_pile(|card| matches!(card.color, Color::Diamond) && card.rank.value() > Rank::Ten.value());

    board.fill_room();
    print!("\x1B[2J\x1B[1;1H");
    board.show();
    while handle_input(&mut board) {
        print!("\x1B[2J\x1B[1;1H");
        board.show();
    }
}
