use crate::cubie_cube::CubieCube;
use crate::pruning_table::PruningTables;
use crate::turn::Turn;

pub struct Solver<'a> {
    tables: &'a PruningTables,
    max_length: u8,
}

impl<'a> Solver<'a> {
    pub fn new(tables: &'a PruningTables) -> Self {
        Self {
            tables,
            max_length: 22, // Standard upper bound for Kociemba (20-22 is typical)
        }
    }

    /// Returns a formatted solution string (e.g., "R U2 D' ...")
    pub fn solve(&mut self, cube: &CubieCube) -> Option<String> {
        let mut solution = Vec::new();

        let mut depth = 0;
        loop {
            // "so_far" tracks moves in the current search stack
            let mut so_far = Vec::new();

            if self.phase1_search(cube, 0, depth, &mut so_far, &mut solution) {
                return Some(self.format_solution(&solution));
            }

            depth += 1;
            if depth > self.max_length {
                return None;
            }
        }
    }

    fn format_solution(&self, moves: &[Turn]) -> String {
        moves
            .iter()
            .map(|m| format!("{:?}", m))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<'a> Solver<'a> {
    fn phase1_search(
        &mut self,
        cube: &CubieCube,
        g: u8,                         // Cost so far (depth)
        bound: u8,                     // Max allowed depth
        path: &mut Vec<Turn>,          // Current path
        full_solution: &mut Vec<Turn>, // Output storage
    ) -> bool {
        // Calculate Heuristic (h)
        // How far are we from the G_1 subgroup?
        let twist = cube.get_twist() as usize;
        let flip = cube.get_flip() as usize;
        let slice = cube.get_slice_sorted() as usize;

        let dist_twist = self.tables.twist_slice_pruning.get(twist * 495 + slice);
        let dist_flip = self.tables.flip_slice_pruning.get(flip * 495 + slice);

        // Use the maximum of the two pruning tables
        let h = std::cmp::max(dist_twist, dist_flip);

        // IDA* Pruning Condition
        // f = g + h. If f > bound, this path is too long.
        if g + h > bound {
            return false;
        }

        // If h == 0, we are inside the G_1 subgroup
        // Now we switch to Phase 2.
        if h == 0 {
            // We found a valid Phase 1 path. Now try to finish with Phase 2.
            // Phase 2 starts with the cube applied with current path.
            // We give it a generous bound (e.g., 10-12 moves) to finish.

            let phase2_bound = self.max_length - g;
            if self.phase2_search(cube, 0, phase2_bound, path, full_solution) {
                return true;
            }

            // If Phase 2 failed, we must backtrack and keep searching Phase 1
            return false;
        }

        // Branching
        let last_move = path.last().cloned();

        for m in Turn::ALL {
            // Apply Redundancy Checks (Reduce to 13-branches)
            if !crate::turn::is_move_allowed(m, last_move) {
                continue;
            }

            // Execute Turn
            let next_cube = cube.multiply(&m.to_cubie());

            path.push(m);
            if self.phase1_search(&next_cube, g + 1, bound, path, full_solution) {
                return true;
            }
            path.pop();
        }

        false
    }

    fn phase2_search(
        &mut self,
        cube: &CubieCube,
        g: u8,
        bound: u8,
        path: &mut Vec<Turn>, // This path continues from Phase 1
        full_solution: &mut Vec<Turn>,
    ) -> bool {
        // Calculate Phase 2 Heuristic
        let cp = cube.get_corner_perm();
        let ud = cube.get_ud_edges();
        let slice = cube.get_slice_perm();

        let dist_cp = self.tables.corner_slice_pruning.get(cp * 24 + slice);
        let dist_ud = self.tables.ud_edge_slice_pruning.get(ud * 24 + slice);

        let h = std::cmp::max(dist_cp, dist_ud);

        if g + h > bound {
            return false;
        }

        // If h == 0 (and checking raw state ensures no collisions), found path
        if cp == 0 && ud == 0 && slice == 0 {
            // Solution Found
            *full_solution = path.clone();
            return true;
        }

        let last_move = path.last().cloned();

        // 3. Branching (G1 Moves Only)
        // U, U2, U3, D, D2, D3, R2, L2, F2, B2
        for &m in Turn::PHASE2_MOVES.iter() {
            if !crate::turn::is_move_allowed(m, last_move) {
                continue;
            }

            let next_cube = cube.multiply(&m.to_cubie());

            path.push(m);
            if self.phase2_search(&next_cube, g + 1, bound, path, full_solution) {
                return true;
            }
            path.pop();
        }

        false
    }
}
