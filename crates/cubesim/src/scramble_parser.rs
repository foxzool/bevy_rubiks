use crate::generic_cube::{CubeSize, Move, Move::*, MoveVariant, MoveVariant::*};
use rand::Rng;

/// Converts a WCA Notation scramble into ``Vec<Move>``.
pub fn parse_scramble(scramble: String) -> Vec<Move> {
    scramble.split_whitespace().map(convert_move).collect()
}

fn convert_move(mv: &str) -> Move {
    let slice = get_slice(mv);
    let variant = get_variant(mv);

    if !mv.contains('w') {
        match &mv[0..1] {
            "U" => U(variant),
            "R" => R(variant),
            "F" => F(variant),
            "L" => L(variant),
            "D" => D(variant),
            "B" => B(variant),
            "x" => X(variant),
            "y" => Y(variant),
            "z" => Z(variant),
            _ => panic!(),
        }
    } else if mv.contains('U') {
        Uw(slice, variant)
    } else if mv.contains('R') {
        Rw(slice, variant)
    } else if mv.contains('F') {
        Fw(slice, variant)
    } else if mv.contains('L') {
        Lw(slice, variant)
    } else if mv.contains('D') {
        Dw(slice, variant)
    } else if mv.contains('B') {
        Bw(slice, variant)
    } else if mv.contains('x') {
        X(variant)
    } else if mv.contains('y') {
        Y(variant)
    } else if mv.contains('z') {
        Z(variant)
    } else {
        panic!()
    }
}

fn get_slice(mv: &str) -> CubeSize {
    if !mv.contains('w') {
        1
    } else {
        mv[0..1].parse::<CubeSize>().unwrap_or(2)
    }
}

fn get_variant(mv: &str) -> MoveVariant {
    if mv.contains('2') {
        Double
    } else if mv.contains('\'') {
        Inverse
    } else {
        Standard
    }
}

/// Recursively merges adjacent moves with the same Move type
/// until no further simplification is possible.
///
/// # Examples
///
/// Simplifying some scrambles:
///
/// ```rust
/// use cubesim::{parse_scramble, simplify_moves};
/// use cubesim::prelude::{Move::*, MoveVariant::*};
///
/// let scramble = parse_scramble(String::from("B B2 B' R B2 B' R2 R' F2"));
/// let simplified = simplify_moves(&scramble);
/// assert_eq!(simplified, vec![B(Double), R(Standard), B(Standard), R(Standard), F(Double)]);
///
/// let scramble = parse_scramble(String::from("R R' U2 F F' U2 x"));
/// let simplified = simplify_moves(&scramble);
/// assert_eq!(simplified, vec![X(Standard)]);
/// ```
pub fn simplify_moves(moves: &[Move]) -> Vec<Move> {
    // Recursively merges adjacent moves of the same Move discriminant
    // until no further simplification is possible.
    use std::mem::discriminant;
    let mut result = vec![];
    if moves.is_empty() {
        return result;
    }

    // keep track of the current move and its amount of clockwise turns
    struct Movement {
        pub mv: Move,
        pub total_turns: u8,
    }
    let mut movement: Movement = Movement {
        mv: moves[0],
        total_turns: moves[0].get_variant() as u8,
    };

    // returns a Move if the simplified movement has any effect on a cube
    fn movement_to_move(m: Movement) -> Option<Move> {
        match m.total_turns % 4 {
            1 => Some(m.mv.with_variant(MoveVariant::Standard)),
            2 => Some(m.mv.with_variant(MoveVariant::Double)),
            3 => Some(m.mv.with_variant(MoveVariant::Inverse)),
            _ => None,
        }
    }

    // merge adjacent moves of the same type
    for mv in moves[1..].iter() {
        if discriminant(&movement.mv) == discriminant(mv) {
            movement.total_turns = (movement.total_turns + mv.get_variant() as u8) % 4;
        } else {
            if let Some(m) = movement_to_move(movement) {
                result.push(m)
            };
            movement = Movement {
                mv: *mv,
                total_turns: mv.get_variant() as u8,
            };
        }
    }
    if let Some(m) = movement_to_move(movement) {
        result.push(m)
    };

    // don't recurse if moves couldn't be simplified further
    if result.len() == moves.len() {
        return result;
    }
    simplify_moves(result.as_slice())
}

pub fn random_scramble(cube_size: CubeSize, has_move_slice: bool) -> Vec<Move> {
    let mut rng = rand::thread_rng();
    let mut scramble = vec![];
    let mut last_move = None;
    let mut last_move_variant = None;
    let mut last_move_slice = None;

    for _ in 0..(cube_size * 10) {
        let mut move_variant: MoveVariant = rand::random();
        let mut move_slice = 1;
        // not gen x y z
        let mut move_type = rng.gen_range(0..=5);

        // don't allow the same move twice in a row
        if let Some(last_move) = last_move {
            if move_type == last_move {
                move_type = (move_type + 1) % 6;
            }
        }

        // don't allow the same move variant twice in a row
        if let Some(last_move_variant) = last_move_variant {
            if move_variant == last_move_variant {
                move_variant = match move_variant {
                    Standard => Inverse,
                    Inverse => Double,
                    Double => Standard,
                }
            }
        }

        // don't allow the same move slice twice in a row
        if let Some(last_move_slice) = last_move_slice {
            if move_slice == last_move_slice {
                move_slice = (move_slice + 1) % cube_size;
            }
        }

        // don't allow the same move slice twice in a row
        if rng.gen_bool(0.5) {
            move_slice = rng.gen_range(1..cube_size);
        }

        let mv = match move_type {
            0 => U(move_variant),
            1 => R(move_variant),
            2 => F(move_variant),
            3 => L(move_variant),
            4 => D(move_variant),
            5 => B(move_variant),
            6 => X(move_variant),
            7 => Y(move_variant),
            8 => Z(move_variant),
            _ => panic!(),
        };

        let mv = if has_move_slice {
            match move_slice {
                1 => mv,

                _ => match mv {
                    U(variant) => Uw(move_slice, variant),
                    R(variant) => Rw(move_slice, variant),
                    F(variant) => Fw(move_slice, variant),
                    L(variant) => Lw(move_slice, variant),
                    D(variant) => Dw(move_slice, variant),
                    B(variant) => Bw(move_slice, variant),
                    X(variant) => X(variant),
                    Y(variant) => Y(variant),
                    Z(variant) => Z(variant),
                    _ => panic!(),
                },
            }
        } else {
            mv
        };

        scramble.push(mv);
        last_move = Some(move_type);
        last_move_variant = Some(move_variant);
        last_move_slice = Some(move_slice);
    }

    scramble
}
