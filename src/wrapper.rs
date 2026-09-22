use axelwas_chess::{Color, EndStates, Move, Piece, PieceTypes, Place, Position};

pub struct Game {
    pub position: Position,
    pub legal: Vec<Move>,
}

impl Game {
    pub fn new() -> Game {
        let position = Position::default();
        let legal = position.all_moves();

        Game {
            position,
            legal,
        }
    }

    pub fn piece_at(&self, place: Place) -> Option<&Piece> {
        return self.position.piece_on(place)
    }

    pub fn turn(&self) -> Color {
        return self.position.turn
    }

    pub fn is_legal(&self, from: Place, to: Place) -> bool {
        let played_move = from.goto(&to);
        return self.legal.contains(&played_move)
    }

    pub fn targets_from(&self, from: Place) -> Vec<Place> {
        let mut targets = Vec::new();
        for row in 0..8 {
            for file in 0..8 {
                let to = Place {
                    row,
                    file
                };
                if self.is_legal(from, to) {
                    targets.push(to);
                }
            }
        }
        return targets
    }

    pub fn play(&mut self, from: Place, to: Place) -> bool {
        let wanttoplay = from.goto(&to);
        if !self.legal.contains(&wanttoplay) { 
            return false; 
        }

        let next = match 
            self.position.clone().execute_move(wanttoplay) {
            Ok(position) => position,
            Err(_) => return false,
        };

        self.position = next;
        self.legal = self.position.all_moves();
        return true
    }

    pub fn status(&self) -> EndStates {
        if !self.legal.is_empty() {
            EndStates::None
        } else if self.position.in_check(self.position.turn) {
            EndStates::Checkmate
        } else {
            EndStates::Stalemate
        }
    }

}
