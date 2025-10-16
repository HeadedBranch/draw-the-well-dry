use crate::ExitStates::{Looping, P1Win, P2Win};
use deck::{Card, Deck};
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufWriter;

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

    let mut valid_lines = Vec::new();
    let mut lines = String::new();
    if let Ok(mut f) = OpenOptions::new().read(true).open("game_results.txt") {
        let _ = f.read_to_string(&mut lines);
        for line in lines.lines() {
            if line.ends_with("P1") {
                wins += 1;
                valid_lines.push(line);
            } else if line.ends_with("P2") {
                valid_lines.push(line);
            } else {
                continue;
            }
            games += 1;
        }
    }
    println!("Save loaded");
    let file = match OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("game_results.txt") {
        Ok(x) => x,
        Err(e) => panic!("{e}"),
    };
    let mut writer = BufWriter::new(file);
    for line in valid_lines {
        writeln!(writer, "{}", line).unwrap();
    }
    writer.flush().unwrap();
    println!("Initialisation complete");
    loop {
        deck.shuffle();
        writer.write_fmt(format_args!("{}", deck)).unwrap();
        let state = handle_game(&deck);
        games += 1;
        match state {
            P1Win => {
                writer.write_all(b"P1\n").unwrap();
                wins += 1;
            }
            P2Win => writer.write_all(b"P2\n").unwrap(),
            Looping => {
                loops += 1;
                #[allow(clippy::collapsible_if)]
                if let Ok(mut file) = File::create("loop_configurations.txt") {
                    if let Err(e) = file.write_all(format!("{:?}", deck.deck()).as_bytes()) {
                        println!("{:?}", deck.deck());
                        println!("{e}");
                    }
                }
            }
        }
        if games.is_multiple_of(1000000) {
            println!(
                "Loops = {}, Games = {} Player 1 win percentage = {}",
                loops,
                games,
                wins as f64 * 100.0 / games as f64
            );
            writer.flush().unwrap();
        }
    }
}

fn handle_game(deck: &Deck) -> ExitStates {
    let mut p1: VecDeque<Card> = VecDeque::new();
    p1.append(&mut VecDeque::from(
        deck.deck()[..(deck.size() / 2)].to_vec(),
    ));
    let mut p2: VecDeque<Card> = VecDeque::new();
    p2.append(&mut VecDeque::from(
        deck.deck()[(deck.size() / 2)..].to_vec(),
    ));
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
            if let Err(e) = handle_penalty(penalty, &mut turn, &mut p1, &mut p2, &mut centre) {
                return e;
            }
        }
    }
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
    use crate::ExitStates::Looping;
    use crate::handle_game;
    use deck::Deck;

    #[test]
    fn test_looping_simple() {
        let deck = Deck::from_str("JS2S2S2SJS2S").unwrap();
        let result = handle_game(&deck);
        assert_eq!(result, Looping);
    }
    #[test]
    fn test_looping_complex() {
        let deck = Deck::from_str("2D2D2DKD2D2D2DQD2DKDQDADJD2D2D2D2D2DADADJD2D2DJD2D2D2D2D2D2D2D2D2D2D2D2DQS2S2S2S2SKSQS2SJS2S2S2S2S2SKSAS").unwrap();
        let result = handle_game(&deck);
        assert_eq!(result, Looping);
    }
}
