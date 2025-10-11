use std::collections::HashSet;
use crate::ExitStates::{Looping, P1Win, P2Win};
use deck::{Card, Deck};
use std::fs::File;
use std::io::prelude::*;

#[derive(Eq, PartialEq, Hash)]
struct GameFrame {
    hand1: Vec<Card>,
    hand2: Vec<Card>,
    turn: bool,
}

#[derive(Eq, PartialEq, Debug)]
enum ExitStates {
    Looping,
    P1Win,
    P2Win,
}

fn main() {
    let mut deck = Deck::new();
    let mut wins: u128 = 0;
    let mut games: u128 = 0;
    let mut loops: u128 = 0;
    loop {
        deck.shuffle();
        let state = handle_game(&deck);
        games += 1;
        if state == P1Win {
            wins += 1;
        } else if state == Looping {
            loops += 1;
            #[allow(clippy::collapsible_if)]
            if let Ok(mut file) = File::open("loop-configurations.txt") {
                if let Err(e) =file.write_all(format!("{:?}",deck.deck()).as_bytes()) {
                    println!("{:?}", deck.deck());
                    println!("{e}");
                }
            }
        }
        println!(
            "Loops = {}, Player 1 win percentage = {}",
            loops,
            wins as f64 * 100.0 / games as f64,
        );
        print!("{}[2J", 27 as char);
    }
}

fn handle_game(deck: &Deck) -> ExitStates {
    let mut p1 = deck.deck()[..(deck.size() / 2)].to_vec();
    let mut p2 = deck.deck()[(deck.size() / 2)..].to_vec();
    let mut turn = true;
    let mut centre: Vec<Card> = Vec::new();
    let mut game_frames = HashSet::<GameFrame>::new();
    loop {
        if p1.is_empty() {
            return P2Win;
        }
        if p2.is_empty() {
            return P1Win;
        }
        let current_frame = GameFrame {
            hand1: p1.clone(),
            hand2: p2.clone(),
            turn,
        };
        if game_frames.contains(&current_frame) {
            return Looping;
        }
        game_frames.insert(current_frame);

        let penalty;
        let card;
        if turn {
            (penalty, card) = check_penalty_card(p1.as_mut());
        } else {
            (penalty, card) = check_penalty_card(p2.as_mut());
        }
        turn = !turn;
        centre.push(card);
        if penalty != 0 {
            #[allow(clippy::collapsible_if)]
            if let Err(e) = handle_penalty(
                penalty,
                &mut turn,
                &mut p1,
                &mut p2,
                &mut centre
            ) {
                return e;
            }
        }
    };
}

fn check_penalty_card(deck: &mut Vec<Card>) -> (u8, Card) {
    let card = deck.pop().expect("Deck is empty");
    (
        match card.value() {
            deck::CardValue::Jack => 1,
            deck::CardValue::Queen => 2,
            deck::CardValue::King => 3,
            deck::CardValue::Ace => 4,
            _ => 0,
        },
        card,
    )
}

fn handle_penalty(
    card_count: u8,
    turn: &mut bool,
    p1: &mut Vec<Card>,
    p2: &mut Vec<Card>,
    centre: &mut Vec<Card>,
) -> Result<(), ExitStates> {
    *turn = !*turn;
    for _ in 0..card_count {
        if p1.is_empty() {
            return Err(P2Win);
        }
        if p2.is_empty() {
            return Err(P1Win);
        }
        let penalty;
        let centre_card;
        if *turn {
            (penalty, centre_card) = check_penalty_card(p2);
        } else {
            (penalty, centre_card) = check_penalty_card(p1);
        }
        centre.push(centre_card);
        if penalty == 0 {
            continue;
        }
        handle_penalty(penalty, turn, p1, p2, centre)?;
        break;
    }
    if *turn {
        p1.reverse();
        p1.append(centre);
        p1.reverse();
    } else {
        p2.reverse();
        p2.append(centre);
        p2.reverse();
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use deck::{Card, Deck};
    use deck::CardValue::*;
    use deck::Suit::*;
    use crate::ExitStates::Looping;
    use crate::handle_game;

    #[test]
    fn test_looping_simple() {
        let mut hand1 = vec![
            Card::new(Jack, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
        ];
        let mut hand2 = vec![
            Card::new(Two, Clubs),
            Card::new(Jack, Clubs),
            Card::new(Two, Clubs),
        ];
        hand1.reverse();
        hand2.reverse();
        let mut deck = Vec::new();
        for card in hand1 {
            deck.push(card);
        }
        for card in hand2 {
            deck.push(card);
        }
        let deck = Deck::new_custom(deck);
        let result = handle_game(&deck);
        assert_eq!(result, Looping);
    }
    #[test]
    fn test_looping_complex(){
        let mut hand1 = vec![
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(King, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Queen, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(King, Diamonds),
            Card::new(Queen, Diamonds),
            Card::new(Ace, Diamonds),
            Card::new(Jack, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Ace, Diamonds),
            Card::new(Ace, Diamonds),
            Card::new(Jack, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Jack, Diamonds),
            Card::new(Two, Diamonds),
            Card::new(Two, Diamonds),
        ];
        let mut hand2 = vec![
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Queen, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(King, Clubs),
            Card::new(Queen, Clubs),
            Card::new(Two, Clubs),
            Card::new(Jack, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
            Card::new(King, Clubs),
            Card::new(Ace, Clubs),
        ];
        let mut deck = Vec::new();
        hand1.reverse();
        hand2.reverse();
        for card in hand1 {
            deck.push(card);
        }
        for card in hand2 {
            deck.push(card);
        }
        let deck = Deck::new_custom(deck);
        let result = handle_game(&deck);
        assert_eq!(result, Looping);
    }
}