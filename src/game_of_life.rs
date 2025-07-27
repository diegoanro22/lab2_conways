#[derive(Clone)]
pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![false; height]; width];
        GameOfLife {
            width,
            height,
            cells,
        }
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cells[x][y] = true;
        }
    }

    pub fn is_alive(&self, x: isize, y: isize) -> bool {
        let w = self.width as isize;
        let h = self.height as isize;
        self.cells[((x + w) % w) as usize][((y + h) % h) as usize]
    }

    pub fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.is_alive(x as isize + dx, y as isize + dy) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut new_cells = self.cells.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                let alive = self.cells[x][y];
                let neighbors = self.count_neighbors(x, y);
                new_cells[x][y] = match (alive, neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        self.cells = new_cells;
    }

    // Glider
    pub fn insert_glider(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Blinker
    pub fn insert_blinker(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [(0, 1), (1, 1), (2, 1)];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Toad
    pub fn insert_toad(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Beacon
    pub fn insert_beacon(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [
            (0, 0), (1, 0), (0, 1), (3, 2), (2, 2), (3, 3),
        ];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Pulsar
    pub fn insert_pulsar(&mut self, offset_x: usize, offset_y: usize) {
    let coords = [
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),

        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];
    for (x, y) in coords {
        self.set_alive(offset_x + x, offset_y + y);
    }
    }

    // Spaceship
    pub fn insert_spaceship(&mut self, offset_x: usize, offset_y: usize
    ) {
        let pattern = [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Acorn
    pub fn insert_acorn(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [(0, 0), (1, 0), (2, 0), (3, 1), (4, 2), (5, 3)];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // R-pentomino
    pub fn insert_r_pentomino(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [(0, 0), (1, 0), (0, 1), (1, 1), (2, 1)];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Glider gun
    pub fn insert_glider_gun(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [
            (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (6, 6),
            (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7),
            (0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (6, 8),
            (2, 5), (3, 5),
            (2, 4), (3, 4),
        ];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Gosper glider gun
    pub fn insert_gosper_glider_gun(&mut self, offset_x: usize , offset_y: usize) {
        let pattern = [
            (0, 24), (1, 24), (2, 24), (3, 24), (4, 24), (5, 24), (6, 24),
            (0, 25), (1, 25), (2, 25), (3, 25), (4, 25), (5, 25), (6, 25),
            (0, 26), (1, 26), (2, 26), (3, 26), (4, 26), (5, 26), (6, 26),
            (2, 23), (3, 23),
            (2, 22), (3, 22),
        ];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Lightweight spaceship (LWSS)
    pub fn insert_lwss(&mut self, offset_x: usize, offset_y: usize) {
    let pattern = [
        (1, 0), (2, 0), (3, 0), (4, 0),
        (0, 1), (4, 1),
        (4, 2),
        (0, 3), (3, 3),
    ];
    for (x, y) in pattern {
        self.set_alive(offset_x + x, offset_y + y);
    }
    
    }

    // Beehive
    pub fn insert_beehive(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [(0, 0), (1, 0), (2, 1), (3, 1), (1, 2), (2, 2)];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Loaf
    pub fn insert_loaf(&mut self, offset_x: usize, offset_y: usize) {
    let pattern = [
        (1, 0), (2, 0),
        (0, 1), (3, 1),
        (1, 2), (3, 2),
        (2, 3),
    ];
    for (x, y) in pattern {
        self.set_alive(offset_x + x, offset_y + y);
    }
    }
    // Boat
    pub fn insert_boat(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [(0, 0), (1, 0), (2, 1), (0, 2), (1, 2)];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Tub
    pub fn insert_tub(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [(0, 0), (1, 0), (0, 1), (1, 1)];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }

    // Medium-weight spaceship (MWSS)
    pub fn insert_mwss(&mut self, offset_x: usize, offset_y: usize) {
    let pattern = [
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
        (0, 1), (5, 1),
        (5, 2),
        (0, 3), (4, 3),
    ];
    for (x, y) in pattern {
        self.set_alive(offset_x + x, offset_y + y);
    }
}

    // Heavyweight spaceship (HWSS)
    pub fn insert_hwss(&mut self, offset_x: usize, offset_y: usize) {
        let pattern = [
            (1, 0), (2, 0), (3, 0), (4, 0), (5, 0),
            (0, 1), (5, 1),
            (5, 2),
            (0, 3), (4, 3),
            (0, 4), (1, 4),
        ];
        for (x, y) in pattern {
            self.set_alive(offset_x + x, offset_y + y);
        }
    }


}
