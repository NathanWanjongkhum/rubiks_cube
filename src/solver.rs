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
            max_length: 22,
        }
    }

    pub fn solve(&mut self, cube: &CubieCube) -> Option<String> {
        let mut best_solution: Option<Vec<Turn>> = None;
        let mut best_length = self.max_length + 1;

        println!("--- Starting Two-Phase Search ---");

        // Phase 1 Iterative Deepening (0 to 12 moves)
        for p1_bound in 0..=12 {
            // THE FIX: Stop outer loop if Phase 1 alone is worse than our best total solve
            if p1_bound >= best_length {
                println!(
                    "Phase 1 bound ({}) exceeded best length ({}). Search complete.",
                    p1_bound,
                    best_length
                );
                break;
            }

            // MONITORING: Show the current search depth
            println!("Searching Phase 1 Depth: {} (Current best: {})", p1_bound, if
                best_length > 22
            {
                "None".to_string()
            } else {
                best_length.to_string()
            });

            let mut path = Vec::new();
            self.phase1_search(cube, 0, p1_bound, &mut path, &mut best_solution, &mut best_length);
        }

        println!("--- Search Finished ---");
        best_solution.map(|s| self.format_solution(&s))
    }

    fn format_solution(&self, moves: &[Turn]) -> String {
        moves
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl<'a> Solver<'a> {
    fn phase1_search(
        &self,
        cube: &CubieCube,
        g: u8,
        p1_bound: u8,
        path: &mut Vec<Turn>,
        best_solution: &mut Option<Vec<Turn>>,
        best_length: &mut u8
    ) {
        let twist = cube.get_twist() as usize;
        let flip = cube.get_flip() as usize;
        let slice = cube.get_slice_sorted() as usize;

        let h1 = std::cmp::max(
            self.tables.twist_slice_pruning.get(twist * 495 + slice),
            self.tables.flip_slice_pruning.get(flip * 495 + slice)
        );

        // Standard Pruning and Global Bound Pruning
        // If this branch mathematically cannot beat our best solution, kill it instantly.
        if g + h1 > p1_bound || g + h1 >= *best_length {
            return;
        }

        // Reached the G1 subgroup at exactly the target Phase 1 depth
        if h1 == 0 && g == p1_bound {
            // Strictly limit Phase 2 to ensure we only find paths SHORTER than our best
            let max_p2 = *best_length - g - 1;

            for p2_bound in 0..=max_p2 {
                let mut p2_path = path.clone();
                if self.phase2_search(cube, 0, p2_bound, &mut p2_path) {
                    let total_length = g + p2_bound;

                    if total_length < *best_length {
                        *best_length = total_length;
                        *best_solution = Some(p2_path.clone());

                        println!(
                            "  -> Found better solution! Length: {:02} | Moves: {}",
                            total_length,
                            self.format_solution(&p2_path)
                        );
                    }

                    break;
                }
            }
        }

        // Halt Phase 1 branching if we hit the Phase 1 depth limit
        if g == p1_bound {
            return;
        }

        let last_move = path.last().cloned();

        for &m in Turn::ALL.iter() {
            if !crate::turn::is_move_allowed(m, last_move) {
                continue;
            }

            let next_cube = cube.multiply(&m.to_cubie());
            path.push(m);
            self.phase1_search(&next_cube, g + 1, p1_bound, path, best_solution, best_length);
            path.pop();
        }
    }

    fn phase2_search(&self, cube: &CubieCube, g: u8, p2_bound: u8, path: &mut Vec<Turn>) -> bool {
        let cp = cube.get_corner_perm();
        let ud = cube.get_ud_edges();
        let slice = cube.get_slice_perm();

        let h2 = std::cmp::max(
            self.tables.corner_slice_pruning.get(cp * 24 + slice),
            self.tables.ud_edge_slice_pruning.get(ud * 24 + slice)
        );

        if g + h2 > p2_bound {
            return false;
        }

        if h2 == 0 && cp == 0 && ud == 0 && slice == 0 {
            return g == p2_bound;
        }

        if g == p2_bound {
            return false;
        }

        let last_move = path.last().cloned();

        for &m in Turn::PHASE2_MOVES.iter() {
            if !crate::turn::is_move_allowed(m, last_move) {
                continue;
            }

            let next_cube = cube.multiply(&m.to_cubie());
            path.push(m);
            if self.phase2_search(&next_cube, g + 1, p2_bound, path) {
                return true;
            }
            path.pop();
        }

        false
    }
}
