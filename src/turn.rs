use crate::cubie_cube::CubieCube;

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
        None => return true,
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
