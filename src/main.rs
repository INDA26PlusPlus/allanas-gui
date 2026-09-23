use axelwas_chess::{Place, Color};
use crate::wrapper::{Game};
mod wrapper;

// helper
fn sq(s: &str) -> Place {
    Place::try_from(s).unwrap()
}

fn main() {
    let mut game = Game::new();

    assert_eq!(game.turn(), Color::White);
    assert_eq!(game.targets_from(sq("e2")).len(), 2);
    println!("{:?}", game.turn());
    println!("{:?}", game.status());
    println!("{:?}", game.position.legal_position());

    assert!(game.play(sq("e2"), sq("e4"), None));
    assert_eq!(game.turn(), Color::Black);
    assert!(!game.play(sq("e4"), sq("e5"), None));
    println!("{:?}", game.turn());
    println!("{:?}", game.status());
    println!("{:?}", game.piece_at(sq("e1")));

    println!("{:?}", game.is_legal(sq("e5"), sq("e2")));
    println!("{:?}", game.is_legal(sq("d0"), sq("e0"))); //panic lol

    return
}