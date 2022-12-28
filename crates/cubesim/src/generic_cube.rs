use std::{
    fmt::{Display, Formatter},
    hash::Hash,
};

pub type CubeSize = i32;

/// A Rubik's Cube of arbitrary size.
///
/// All implementors of this trait are (externally) immutable and persistent.
/// Methods that involve mutating a Rubik's Cube will instead return a new
/// Cube with the mutation applied, leaving the old Cube intact.
pub trait Cube: Clone + Eq + Hash + PartialEq {
    /// Creates a solved cube of the given size.
    fn new(size: CubeSize) -> Self;

    /// The size of the cube.
    fn size(&self) -> CubeSize;

    /// A one-dimensional representation of a cube as a sequence of the faces.
    ///
    /// # Examples
    ///
    /// Solved 3x3x3 cube:
    ///
    /// ```rust
    /// use cubesim::prelude::{Cube, Face::*};
    /// use cubesim::FaceletCube;
    ///
    /// let cube = FaceletCube::new(3);
    /// assert_eq!(cube.state(), vec![
    ///     U, U, U, U, U, U, U, U, U,
    ///     R, R, R, R, R, R, R, R, R,
    ///     F, F, F, F, F, F, F, F, F,
    ///     D, D, D, D, D, D, D, D, D,
    ///     L, L, L, L, L, L, L, L, L,
    ///     B, B, B, B, B, B, B, B, B
    /// ]);
    /// ```
    fn state(&self) -> Vec<Face>;

    /// Whether a cube is solved.
    fn is_solved(&self) -> bool {
        fn all_equal<T: Clone + PartialEq>(arr: &[T]) -> bool {
            arr.iter().all(|x| *x == arr[0])
        }

        let face_length = (self.size() * self.size()) as usize;
        let state = self.state();

        let mut is_solved = true;
        for i in 0..6 {
            let face_start = i * face_length;
            let face_end = face_start + face_length;

            is_solved = is_solved && all_equal(&state[face_start..face_end]);
        }

        is_solved
    }

    /// Replaces each piece of the cube according to the given mapping function.
    /// This is useful for defining custom solvers by replacing certain pieces
    /// in order to reduce the search space.
    ///
    /// # Examples
    ///
    /// Cross Mask
    ///
    /// ```rust
    /// use cubesim::prelude::{Cube, Face::*, Move, MoveVariant};
    /// use cubesim::FaceletCube;
    /// use cubesim::sticker_index;
    ///
    /// let cross_pieces = [
    ///     sticker_index(3, D, 2), sticker_index(3, D, 4),
    ///     sticker_index(3, D, 6), sticker_index(3, D, 8),
    /// ];
    ///
    /// let masked_cube = FaceletCube::new(3).mask(&|i, f| if cross_pieces.contains(&i) { f } else { X });
    /// assert_eq!(masked_cube.state(), vec![
    ///      X, X, X, X, X, X, X, X, X,
    ///      X, X, X, X, X, X, X, X, X,
    ///      X, X, X, X, X, X, X, X, X,
    ///      X, D, X, D, X, D, X, D, X,
    ///      X, X, X, X, X, X, X, X, X,
    ///      X, X, X, X, X, X, X, X, X
    /// ]);
    /// ```
    fn mask(&self, mask: &dyn Fn(CubeSize, Face) -> Face) -> Self;

    /// Apply a move to a cube.
    ///
    /// # Examples
    ///
    /// Rotate the upper layer by 90 degrees:
    ///
    /// ```rust
    /// use cubesim::prelude::{Cube, Face::*, Move, MoveVariant};
    /// use cubesim::FaceletCube;
    ///
    /// let solved_cube = FaceletCube::new(3);
    /// let turned_cube = solved_cube.apply_move(Move::U(MoveVariant::Standard));
    /// assert_eq!(turned_cube.state(), vec![
    ///     U, U, U, U, U, U, U, U, U,
    ///     B, B, B, R, R, R, R, R, R,
    ///     R, R, R, F, F, F, F, F, F,
    ///     D, D, D, D, D, D, D, D, D,
    ///     F, F, F, L, L, L, L, L, L,
    ///     L, L, L, B, B, B, B, B, B
    /// ]);
    /// ```
    fn apply_move(&self, mv: Move) -> Self;

    /// Apply a sequence of moves to a cube.
    ///
    /// # Examples
    ///
    /// Rotate the upper layer by 90 degrees:
    ///
    /// ```rust
    /// use cubesim::prelude::{Cube, Face::*, Move, MoveVariant};
    /// use cubesim::FaceletCube;
    ///
    /// let solved_cube = FaceletCube::new(3);
    /// let turned_cube = solved_cube.apply_moves(&vec![
    ///     Move::U(MoveVariant::Standard),
    ///     Move::R(MoveVariant::Double),
    ///     Move::B(MoveVariant::Inverse),
    /// ]);
    /// assert_eq!(turned_cube.state(), vec![
    ///     L, L, F, U, U, D, U, U, D,
    ///     R, R, U, R, R, U, B, B, D,
    ///     R, R, B, F, F, B, F, F, L,
    ///     D, D, U, D, D, U, B, R, R,
    ///     D, F, F, D, L, L, U, L, L,
    ///     L, B, B, L, B, B, F, F, R
    /// ]);
    /// ```
    fn apply_moves(&self, mvs: &[Move]) -> Self
    where
        Self: Sized,
    {
        let mut cube = self.clone();

        for mv in mvs {
            cube = cube.apply_move(*mv);
        }

        cube
    }
}

use derive_more::Display;

/// A face of a Rubik's Cube sticker represented in WCA notation.
///
/// The faces follow the standard WCA notation as described in the [WCA regulations].
///
/// [WCA regulations]: worldcubeassociation.org/regulations/#article-12-notation
#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum Face {
    /// Upper face.
    U,
    /// Left face.
    L,
    /// Front face.
    F,
    /// Right face.
    R,
    /// Back face.
    B,
    /// Down face.
    D,
    /// Masked face. Represents a placeholder sticker.
    X,
}

/// A designated ordering of the faces.
pub const ORDERED_FACES: [Face; 6] = [Face::U, Face::R, Face::F, Face::D, Face::L, Face::B];

/// Get the index of a specific piece on a specific face.
///
/// # Examples
///
/// 1st piece on the front face of a 3x3x3 cube:
///
/// ```rust
/// use cubesim::prelude::{Cube, Face};
/// use cubesim::sticker_index;
///
/// assert_eq!(sticker_index(3, Face::F, 1), 18);
/// ```
pub fn sticker_index(size: CubeSize, face: Face, index: CubeSize) -> CubeSize {
    (ORDERED_FACES.iter().position(|&f| f == face).unwrap() as CubeSize) * size * size + index
        - 1 as CubeSize
}

/// A move of a NxNxN Rubik's Cube represented in WCA notation.
///
/// Each Move must be tagged with a ``MoveVariant`` to completely define a move.
///
/// The moves follow the standard WCA notation as described in the [WCA regulations].
///
/// [WCA regulations]: worldcubeassociation.org/regulations/#article-12-notation
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    /// Rotate the upper layer.
    U(MoveVariant),
    /// Rotate the left layer.
    L(MoveVariant),
    /// Rotate the front layer.
    F(MoveVariant),
    /// Rotate the right layer.
    R(MoveVariant),
    /// Rotate the back layer.
    B(MoveVariant),
    /// Rotate the down layer.
    D(MoveVariant),
    /// Rotate the uppermost n layers.
    Uw(CubeSize, MoveVariant),
    /// Rotate the leftmost n layers.
    Lw(CubeSize, MoveVariant),
    /// Rotate the frontmost n layers.
    Fw(CubeSize, MoveVariant),
    /// Rotate the rightmost n layers.
    Rw(CubeSize, MoveVariant),
    /// Rotate the backmost n layers.
    Bw(CubeSize, MoveVariant),
    /// Rotate the downmost n layers.
    Dw(CubeSize, MoveVariant),
    /// Rotate the entire cube along the x-axis.
    X(MoveVariant),
    /// Rotate the entire cube along the y-axis.
    Y(MoveVariant),
    /// Rotate the entire cube along the z-axis.
    Z(MoveVariant),
}

impl Move {
    /// Extracts the MoveVariant of a Move.
    pub fn get_variant(&self) -> MoveVariant {
        match self {
            Move::U(v)
            | Move::L(v)
            | Move::F(v)
            | Move::R(v)
            | Move::B(v)
            | Move::D(v)
            | Move::X(v)
            | Move::Y(v)
            | Move::Z(v)
            | Move::Uw(_, v)
            | Move::Lw(_, v)
            | Move::Fw(_, v)
            | Move::Rw(_, v)
            | Move::Bw(_, v)
            | Move::Dw(_, v) => *v,
        }
    }

    /// Returns the Move with the given MoveVariant.
    pub fn with_variant(&self, variant: MoveVariant) -> Move {
        match self {
            Move::U(_) => Move::U(variant),
            Move::L(_) => Move::L(variant),
            Move::F(_) => Move::F(variant),
            Move::R(_) => Move::R(variant),
            Move::B(_) => Move::B(variant),
            Move::D(_) => Move::D(variant),
            Move::Uw(n, _) => Move::Uw(*n, variant),
            Move::Lw(n, _) => Move::Lw(*n, variant),
            Move::Fw(n, _) => Move::Fw(*n, variant),
            Move::Rw(n, _) => Move::Rw(*n, variant),
            Move::Bw(n, _) => Move::Bw(*n, variant),
            Move::Dw(n, _) => Move::Dw(*n, variant),
            Move::X(_) => Move::X(variant),
            Move::Y(_) => Move::Y(variant),
            Move::Z(_) => Move::Z(variant),
        }
    }

    fn get_move_name(&self) -> String {
        match self {
            Move::U(_) => "U".to_string(),
            Move::L(_) => "L".to_string(),
            Move::F(_) => "F".to_string(),
            Move::R(_) => "R".to_string(),
            Move::B(_) => "B".to_string(),
            Move::D(_) => "D".to_string(),
            Move::Uw(n, _) => {
                if *n == 2 {
                    "Uw".to_string()
                } else {
                    format!("{n}Uw")
                }
            }
            Move::Lw(n, _) => {
                if *n == 2 {
                    "Lw".to_string()
                } else {
                    format!("{n}Lw")
                }
            }
            Move::Fw(n, _) => {
                if *n == 2 {
                    "Fw".to_string()
                } else {
                    format!("{n}Fw")
                }
            }
            Move::Rw(n, _) => {
                if *n == 2 {
                    "Rw".to_string()
                } else {
                    format!("{n}Rw")
                }
            }
            Move::Bw(n, _) => {
                if *n == 2 {
                    "Bw".to_string()
                } else {
                    format!("{n}Bw")
                }
            }
            Move::Dw(n, _) => {
                if *n == 2 {
                    "Dw".to_string()
                } else {
                    format!("{n}Dw")
                }
            }
            Move::X(_) => "X".to_string(),
            Move::Y(_) => "Y".to_string(),
            Move::Z(_) => "Z".to_string(),
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.get_move_name(), self.get_variant())
    }
}

/// A move variation that must be applied to the ```Move``` struct.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveVariant {
    /// A 90 degree clockwise turn.
    Standard = 1,
    /// A 180 degree clockwise turn.
    Double,
    /// A 90 degree counter-clockwise turn.
    Inverse,
}

impl Display for MoveVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MoveVariant::Standard => "",
            MoveVariant::Double => "2",
            MoveVariant::Inverse => "'",
        };
        write!(f, "{s}")
    }
}

/// Get the solved state for a cube of a given size.
pub fn solved_state(size: CubeSize) -> Vec<Face> {
    ORDERED_FACES
        .iter()
        .flat_map(|&face| vec![face; (size * size) as usize])
        .collect()
}

/// Get all possible moves for a cube of a given size.
pub fn all_moves(size: CubeSize) -> Vec<Move> {
    use Move::*;
    use MoveVariant::*;

    let mut moveset = Vec::new();

    for mv in [U, R, F, L, D, B] {
        for variant in [Standard, Double, Inverse] {
            moveset.push(mv(variant));
        }
    }

    for mv in [Uw, Lw, Fw, Rw, Bw, Dw] {
        for variant in [Standard, Double, Inverse] {
            for slice in 1..=(size / 2) {
                moveset.push(mv(slice, variant));
            }
        }
    }

    moveset
}
