use std::fmt::Display;
use std::io::{self, Write, Read};
use rand::{rng, seq::SliceRandom};

#[derive(Debug, PartialEq)]
enum KeyPress {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Char(char),
    Unknown,
}

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


#[derive(PartialEq, Clone)]
enum Selection {
    DrawPile,
    TablePile(usize),
    PlayerHand(usize),
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
    pub selection: Option<Selection>,
}


impl Board {
    fn new() -> Self {
        Board {
            draw_pile: Vec::new(),
            table_pile: Vec::new(),
            discard_pile: Vec::new(),
            player_hand: Vec::new(),
            player_hp: 20,
            selection: None,
        }
    }

    fn setup(&mut self) {
        self.draw_pile = Card::deck();
        self.draw_pile.shuffle(&mut rand::rng());
    }

    fn show(&self) {
        if self.selection == Some(Selection::DrawPile) {
            println!("left: [{}] ", self.draw_pile.len());
        } else {
            println!("left:  {}  ", self.draw_pile.len());
        }
        print!("room: ");
        for (i, card) in self.table_pile.iter().enumerate() {
            if self.selection == Some(Selection::TablePile(i)) {
                print!("[{}] ", card);
                continue;
            }
            print!(" {}  ", card);
        }
        println!();
        print!("{:2}hp  ", self.player_hp);

        for (i, card) in self.player_hand.iter().enumerate() {
            if self.selection == Some(Selection::PlayerHand(i)) {
                print!("[{}] ", card);
                continue;
            }
            print!(" {}  ", card);
        }
        println!();
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



fn read_key() -> io::Result<KeyPress> {
    let mut buffer = [0; 1];
    io::stdin().read_exact(&mut buffer)?;
    
    match buffer[0] {
        b'\n' | b'\r' => Ok(KeyPress::Enter),
        b'\x1b' => {
            // Escape sequence
            let mut seq = [0; 2];
            if io::stdin().read_exact(&mut seq).is_ok() {
                if seq[0] == b'[' {
                    match seq[1] {
                        b'A' => Ok(KeyPress::Up),
                        b'B' => Ok(KeyPress::Down),
                        b'C' => Ok(KeyPress::Right),
                        b'D' => Ok(KeyPress::Left),
                        _ => Ok(KeyPress::Unknown),
                    }
                } else {
                    Ok(KeyPress::Unknown)
                }
            } else {
                Ok(KeyPress::Unknown)
            }
        }
        c if c.is_ascii() => Ok(KeyPress::Char(c as char)),
        _ => Ok(KeyPress::Unknown),
    }
}

fn handle_input(board: &mut Board) -> bool {
    print!("\x1B[2J\x1B[1;1H");
    board.show();

    match read_key() {
        Ok(KeyPress::Left) => {
            move_selection_left(board);
        }
        Ok(KeyPress::Right) => {
            move_selection_right(board);
        }
        Ok(KeyPress::Up) => {
            move_selection_up(board);
        }
        Ok(KeyPress::Down) => {
            move_selection_down(board);
        }
        Ok(KeyPress::Enter) => {
            handle_enter_action(board);
        }
        Ok(KeyPress::Char('q')) => {
            return false;
        }
        Ok(KeyPress::Char('?')) => {
            show_help();
        }
        Ok(KeyPress::Char('e')) => {
            handle_equip_action(board);
        }
        Ok(KeyPress::Char('a')) => {
            handle_attack_action(board);
        }
        Ok(KeyPress::Char('w')) => {
            handle_weapon_attack_action(board);
        }
        Ok(KeyPress::Char('h')) => {
            handle_heal_action(board);
        }
        Ok(KeyPress::Char('d')) => {
            handle_discard_action(board);
        }
        Ok(KeyPress::Char('r')) => {
            handle_run_action(board);
        }
        Ok(KeyPress::Char('f')) => {
            board.fill_room();
        }
        _ => {}
    }

    true
}


fn move_selection_left(board: &mut Board) {
    match &board.selection {
        None => {
            board.selection = Some(Selection::DrawPile);
        }
        Some(Selection::DrawPile) => {

        }
        Some(Selection::TablePile(i)) => {
            if *i > 0 {
                board.selection = Some(Selection::TablePile(i - 1));
            } else {
                board.selection = Some(Selection::DrawPile);
            }
        }
        Some(Selection::PlayerHand(i)) => {
            if *i > 0 {
                board.selection = Some(Selection::PlayerHand(i - 1));
            } else if !board.table_pile.is_empty() {
                board.selection = Some(Selection::TablePile(board.table_pile.len() - 1));
            } else {
                board.selection = Some(Selection::DrawPile);
            }
        }
    }
}

fn move_selection_right(board: &mut Board) {
    match &board.selection {
        None => {
            board.selection = Some(Selection::DrawPile);
        }
        Some(Selection::DrawPile) => {
            if !board.table_pile.is_empty() {
                board.selection = Some(Selection::TablePile(0));
            } else if !board.player_hand.is_empty() {
                board.selection = Some(Selection::PlayerHand(0));
            }
        }
        Some(Selection::TablePile(i)) => {
            if *i < board.table_pile.len() - 1 {
                board.selection = Some(Selection::TablePile(i + 1));
            } else if !board.player_hand.is_empty() {
                board.selection = Some(Selection::PlayerHand(0));
            }
        }
        Some(Selection::PlayerHand(i)) => {
            if *i < board.player_hand.len() - 1 {
                board.selection = Some(Selection::PlayerHand(i + 1));
            }
        }
    }
}

fn move_selection_up(board: &mut Board) {
    match &board.selection {
        None => {
            board.selection = Some(Selection::DrawPile);
        }
        Some(Selection::PlayerHand(i)) => {
            if !board.table_pile.is_empty() {
                let new_index = (*i).min(board.table_pile.len() - 1);
                board.selection = Some(Selection::TablePile(new_index));
            } else {
                board.selection = Some(Selection::DrawPile);
            }
        }
        Some(Selection::TablePile(_)) => {
            board.selection = Some(Selection::DrawPile);
        }
        _ => {}
    }
}

fn move_selection_down(board: &mut Board) {
    match &board.selection {
        None => {
            board.selection = Some(Selection::DrawPile);
        }
        Some(Selection::DrawPile) => {
            if !board.table_pile.is_empty() {
                board.selection = Some(Selection::TablePile(0));
            }
        }
        Some(Selection::TablePile(i)) => {
            if !board.player_hand.is_empty() {
                let new_index = (*i).min(board.player_hand.len() - 1);
                board.selection = Some(Selection::PlayerHand(new_index));
            }
        }
        _ => {}
    }
}



fn handle_enter_action(board: &mut Board) {
    match &board.selection {
        Some(Selection::DrawPile) => {
            board.fill_room();
        }
        _ => {}
    }
}

fn handle_equip_action(board: &mut Board) {
    if let Some(Selection::TablePile(i)) = &board.selection {
        let index = *i;
        if index < board.table_pile.len() {
            let card = board.table_pile.remove(index);
            board.equip_weapon(card);
        }
    }
}

fn handle_attack_action(board: &mut Board) {
    if let Some(Selection::TablePile(i)) = &board.selection {
        let index = *i;
        if index < board.table_pile.len() {
            board.attack_no_weapon(index);
        }
    }
}

fn handle_weapon_attack_action(board: &mut Board) {
    if let Some(Selection::TablePile(i)) = &board.selection {
        let index = *i;
        if index < board.table_pile.len() {
            board.attack_with_weapon(index);
        }
    }
}

fn handle_heal_action(board: &mut Board) {
    if let Some(Selection::TablePile(i)) = &board.selection {
        let index = *i;
        if index < board.table_pile.len() {
            let card = board.table_pile.remove(index);
            board.player_hp += card.rank.value();
            board.discard_pile.push(card);
        }
    }
}

fn handle_discard_action(board: &mut Board) {
    match &board.selection {
        Some(Selection::TablePile(i)) => {
            let index = *i;
            if index < board.table_pile.len() {
                let card = board.table_pile.remove(index);
                board.discard_pile.push(card);
            }
        }
        Some(Selection::PlayerHand(_)) => {
            board.discard_player_hand();
        }
        _ => {}
    }
}

fn handle_run_action(board: &mut Board) {
    board.table_pile.shuffle(&mut rng());
    for _ in 0..=3 {
        if let Some(card) = board.table_pile.pop() {
            board.draw_pile.insert(0, card);
        }
    }
    board.fill_room();
}


fn show_help() {
    println!("\n=== CONTROLS ===");
    println!("Arrow Keys - Navigate");
    println!("Enter - Select");
    println!("f - Fill room");
    println!("h - Help");
    println!("q - Quit");
    println!("\n=== ACTIONS (when card selected) ===");
    println!("e - Equip weapon");
    println!("a - Attack without weapon");
    println!("w - Attack with weapon");
    println!("h - Heal");
    println!("d - Discard");
    println!();
}


fn main() {
    #[cfg(unix)]
    {
        use std::process::Command;
        Command::new("stty")
            .args(&["-icanon", "-echo"])
            .status()
            .expect("Failed to set terminal mode");
    }


    let mut board = Board::new();
    board.setup();

    board.filter_draw_pile(|card| matches!(card.color, Color::Diamond) && card.rank.value() > Rank::Ten.value());

    board.fill_room();
    while handle_input(&mut board) {
        if board.player_hp == 0 {
            println!("You have been defeated!");
            break;
        }
        if board.draw_pile.is_empty() && board.table_pile.is_empty() {
            println!("You have cleared all cards! Victory!");
            break;
        }
    }

    
    #[cfg(unix)]
    {
        use std::process::Command;
        Command::new("stty")
            .args(&["icanon", "echo"])
            .status()
            .expect("Failed to restore terminal mode");
    }
    }

