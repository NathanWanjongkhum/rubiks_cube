use crate::cubie_cube::CubieCube;

use std::fmt;

/// Represents the 18 possible moves in Half-Turn Metric
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    U,
    U2,
    U3,
    R,
    R2,
    R3,
    F,
    F2,
    F3,
    D,
    D2,
    D3,
    L,
    L2,
    L3,
    B,
    B2,
    B3,
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Turn::U => "U",
            Turn::U2 => "U2",
            Turn::U3 => "U'",
            Turn::D => "D",
            Turn::D2 => "D2",
            Turn::D3 => "D'",
            Turn::R => "R",
            Turn::R2 => "R2",
            Turn::R3 => "R'",
            Turn::L => "L",
            Turn::L2 => "L2",
            Turn::L3 => "L'",
            Turn::F => "F",
            Turn::F2 => "F2",
            Turn::F3 => "F'",
            Turn::B => "B",
            Turn::B2 => "B2",
            Turn::B3 => "B'",
        };
        write!(f, "{}", s)
    }
}

impl Turn {
    pub const ALL: [Turn; 18] = [
        Turn::U,
        Turn::U2,
        Turn::U3,
        Turn::R,
        Turn::R2,
        Turn::R3,
        Turn::F,
        Turn::F2,
        Turn::F3,
        Turn::D,
        Turn::D2,
        Turn::D3,
        Turn::L,
        Turn::L2,
        Turn::L3,
        Turn::B,
        Turn::B2,
        Turn::B3,
    ];

    pub const PHASE2_MOVES: [Turn; 10] = [
        Turn::U,
        Turn::U2,
        Turn::U3,
        Turn::D,
        Turn::D2,
        Turn::D3,
        Turn::R2,
        Turn::L2,
        Turn::F2,
        Turn::B2,
    ];

    /// Returns the CubieCube representation for ANY move.
    /// This generalizes the logic: Base moves are looked up,
    /// Derived moves (X2, X') are calculated via group multiplication.
    pub fn to_cubie(&self) -> CubieCube {
        let base = match self {
            Turn::U | Turn::U2 | Turn::U3 => CubieCube::U,
            Turn::R | Turn::R2 | Turn::R3 => CubieCube::R,
            Turn::F | Turn::F2 | Turn::F3 => CubieCube::F,
            Turn::D | Turn::D2 | Turn::D3 => CubieCube::D,
            Turn::L | Turn::L2 | Turn::L3 => CubieCube::L,
            Turn::B | Turn::B2 | Turn::B3 => CubieCube::B,
        };

        match self {
            // Base moves (Quarter turns)
            Turn::U | Turn::R | Turn::F | Turn::D | Turn::L | Turn::B => base,

            // Double turns (Half-turn metric)
            Turn::U2 | Turn::R2 | Turn::F2 | Turn::D2 | Turn::L2 | Turn::B2 => base.multiply(&base),

            // Inverse turns (Prime moves)
            Turn::U3 | Turn::R3 | Turn::F3 | Turn::D3 | Turn::L3 | Turn::B3 => {
                base.multiply(&base.multiply(&base))
            }
        }
    }

    /// Returns the "axis" of the move (0=UD, 1=LR, 2=FB)
    pub fn axis(&self) -> u8 {
        match self {
            Turn::U | Turn::U2 | Turn::U3 | Turn::D | Turn::D2 | Turn::D3 => 0,
            Turn::L | Turn::L2 | Turn::L3 | Turn::R | Turn::R2 | Turn::R3 => 1,
            Turn::F | Turn::F2 | Turn::F3 | Turn::B | Turn::B2 | Turn::B3 => 2,
        }
    }

    /// Returns the "face" index (0..5) to check priority
    pub fn face(&self) -> u8 {
        match self {
            Turn::U | Turn::U2 | Turn::U3 => 0,
            Turn::D | Turn::D2 | Turn::D3 => 1,
            Turn::L | Turn::L2 | Turn::L3 => 2,
            Turn::R | Turn::R2 | Turn::R3 => 3,
            Turn::F | Turn::F2 | Turn::F3 => 4,
            Turn::B | Turn::B2 | Turn::B3 => 5,
        }
    }
}

/// Returns TRUE if the move is allowed (valid).
pub fn is_move_allowed(current_move: Turn, last: Option<Turn>) -> bool {
    let last_turn = match last {
        Some(t) => t,
        None => {
            return true;
        }
    };

    let curr_face = current_move.face();
    let last_face = last_turn.face();

    // Don't move the same face twice (e.g., F F)
    if curr_face == last_face {
        return false;
    }

    // Commutative Redundancy (Opposite Faces)
    // Check if they are on the same axis (e.g., U and D)
    if current_move.axis() == last_turn.axis() {
        // We strictly enforce an order: Upper Index must come BEFORE Lower Index.
        // If we try to do (Lower Index) -> (Upper Index), we allow it.
        // If we try to do (Upper Index) -> (Lower Index), we BLOCK it.
        // Example: Allow U(0) -> D(1). Block D(1) -> U(0).
        if curr_face < last_face {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_move_cycles() {
        for m in Turn::ALL {
            let m_cubie = m.to_cubie();

            let m2 = m_cubie.multiply(&m_cubie);

            match m {
                // Half-turns should equal to identity after exactly 2 actions
                Turn::U2 | Turn::R2 | Turn::F2 | Turn::D2 | Turn::L2 | Turn::B2 => {
                    assert_eq!(
                        m2,
                        CubieCube::SOLVED,
                        "Half-turn {:?} failed the order-2 identity cycle test!",
                        m
                    );
                }
                // Quarter and Prime turns should equal to identity after 4 actions
                _ => {
                    let m4 = m2.multiply(&m2);
                    assert_eq!(
                        m4,
                        CubieCube::SOLVED,
                        "Quarter/Prime turn {:?} failed the order-4 identity cycle test!",
                        m
                    );
                }
            }
        }
    }

    #[test]
    fn test_coordinate_bijection() {
        // Coordinate encoding and decoding is symmetric
        // for all 18 moves. This verifies the coordinate math.
        for m in Turn::ALL {
            let state = m.to_cubie();

            // Phase 1 Bijection
            let twist = state.get_twist();
            assert_eq!(
                CubieCube::set_twist(twist).co,
                state.co,
                "Twist coordinate symmetry failed for move {:?}",
                m
            );

            let flip = state.get_flip();
            assert_eq!(
                CubieCube::set_flip(flip).eo,
                state.eo,
                "Flip coordinate symmetry failed for move {:?}",
                m
            );

            // Phase 2 Bijection (Permutations are only valid inside G1 subgroup)
            if Turn::PHASE2_MOVES.contains(&m) {
                let cp = state.get_corner_perm();
                assert_eq!(
                    CubieCube::set_corner_perm(cp).cp,
                    state.cp,
                    "Corner Permutation symmetry failed for move {:?}",
                    m
                );

                let ud = state.get_ud_edges();
                let state_ud = CubieCube::set_ud_edges(ud);
                assert_eq!(
                    state_ud.ep[0..8],
                    state.ep[0..8],
                    "UD Edge Permutation symmetry failed for move {:?}",
                    m
                );

                let slice = state.get_slice_perm();
                let state_slice = CubieCube::set_slice_perm(slice);
                assert_eq!(
                    state_slice.ep[8..12],
                    state.ep[8..12],
                    "Slice Permutation symmetry failed for move {:?}",
                    m
                );
            }
        }
    }
}
