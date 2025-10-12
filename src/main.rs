use std::collections::{HashSet, VecDeque};
use crate::ExitStates::{Looping, P1Win, P2Win};
use deck::{Card, Deck};
use std::fs::File;
use std::io::prelude::*;

#[derive(Eq, PartialEq, Hash)]
struct GameFrame {
    hand1: VecDeque<Card>,
    hand2: VecDeque<Card>,
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
            "Loops = {}, Games = {} Player 1 win percentage = {}",
            loops,
            games,
            wins as f64 * 100.0 / games as f64
        );
        print!("{}[2J", 27 as char);
    }
}

fn handle_game(deck: &Deck) -> ExitStates {
    let mut p1: VecDeque<Card> = VecDeque::new();
    p1.append(&mut VecDeque::from(deck.deck()[..(deck.size() / 2)].to_vec()));
    let mut p2: VecDeque<Card> = VecDeque::new();
    p2.append(&mut VecDeque::from(deck.deck()[(deck.size() / 2)..].to_vec()));
    let mut turn = true;
    let mut centre: VecDeque<Card> = VecDeque::new();
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
            (penalty, card) = check_penalty_card(&mut p1);
        } else {
            (penalty, card) = check_penalty_card(&mut p2);
        }

        turn = !turn;
        centre.push_back(card);
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

fn check_penalty_card(deck: &mut VecDeque<Card>) -> (u8, Card) {
    let card = deck.pop_front().expect("Deck is empty");
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
    p1: &mut VecDeque<Card>,
    p2: &mut VecDeque<Card>,
    centre: &mut VecDeque<Card>,
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
        centre.push_back(centre_card);
        if penalty == 0 {
            continue;
        }
        handle_penalty(penalty, turn, p1, p2, centre)?;
        break;
    }
    if *turn {
        p1.append(centre);
    } else {
        p2.append(centre);
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
        let hand1 = vec![
            Card::new(Jack, Clubs),
            Card::new(Two, Clubs),
            Card::new(Two, Clubs),
        ];
        let hand2 = vec![
            Card::new(Two, Clubs),
            Card::new(Jack, Clubs),
            Card::new(Two, Clubs),
        ];
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
        let hand1 = vec![
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
        let hand2 = vec![
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