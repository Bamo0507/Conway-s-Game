use crate::framebuffer::Framebuffer;

// Tamanio del juego 100x100
pub const WIDTH: usize = 100;
pub const HEIGHT: usize = 100;

// Manejar grid actual, y el siguiente para la logica del juego
pub struct Life {
    pub grid: [[bool; WIDTH]; HEIGHT],
    pub next: [[bool; WIDTH]; HEIGHT],
}

impl Life {
    pub fn new() -> Self {
        let mut life = Life {
            grid: [[false; WIDTH]; HEIGHT],
            next: [[false; WIDTH]; HEIGHT],
        };
        life.seed(); // grid inicial
        life
    }

    fn seed(&mut self) {
        let center_x = WIDTH / 2;
        let center_y = HEIGHT / 2;

        // Pulsares haciendo una cruz
        self.add_pulsar(center_x, center_y);
        self.add_pulsar(center_x, center_y.saturating_sub(30));
        self.add_pulsar(center_x, (center_y + 30).min(HEIGHT - 15));
        self.add_pulsar(center_x.saturating_sub(30), center_y);
        self.add_pulsar((center_x + 30).min(WIDTH - 15), center_y);

        // Tubs entre centro y pulsar
        let tub_midpoints = [
            (center_x - 15, center_y),
            (center_x + 15, center_y),
            (center_x, center_y - 15),
            (center_x, center_y + 15),
        ];
        for (mx, my) in tub_midpoints {
            if mx > 1 && my > 1 && mx + 1 < WIDTH && my + 1 < HEIGHT {
                self.add_tub(mx - 1, my - 1);
            }
        }

        // Toads en esquinas
        let cmx = 4usize;
        let cmy = 4usize;
        self.add_toad(cmx, cmy);
        self.add_toad(WIDTH - cmx - 4, cmy);
        self.add_toad(cmx, HEIGHT - cmy - 2);
        self.add_toad(WIDTH - cmx - 4, HEIGHT - cmy - 2);

        // Beehives en diamante por esquina
        let corner_beehive_anchors = [
            (0usize, 0usize),          
            (WIDTH - 26, 0usize),      
            (0usize, HEIGHT - 26),     
            (WIDTH - 26, HEIGHT - 26), 
        ];
        let top_offsets = [
            (12usize, 6usize),  
            (8usize, 12usize),  
            (16usize, 12usize), 
            (12usize, 18usize), 
        ];
        let bottom_offsets = [
            (12usize, 2usize),  
            (8usize, 8usize),
            (16usize, 8usize),
            (12usize, 14usize),
        ];
        for (ax, ay) in corner_beehive_anchors {
            let use_top = ay == 0;
            let offsets = if use_top { &top_offsets } else { &bottom_offsets };
            for (ox, oy) in offsets {
                let bx = ax + *ox;
                let by = ay + *oy;
                if bx + 3 < WIDTH && by + 2 < HEIGHT {
                    self.add_beehive(bx, by);
                }
            }
        }

        // Diamante de blinkers
        // En diagonal hacia el centro de los beehives
        let blinker_diamond_centers = [
            (32usize, 32usize),                 
            (WIDTH - 32, 32usize),              
            (32usize, HEIGHT - 32),             
            (WIDTH - 32, HEIGHT - 32),          
        ];
        let gap_x: isize = 6;
        let gap_y: isize = 6;

        for (cx, cy) in blinker_diamond_centers {
            let cx_i = cx as isize;
            let cy_i = cy as isize;

            let top_base_y = cy_i - gap_y - 1; 
            if top_base_y >= 0 && top_base_y + 2 < HEIGHT as isize {
                self.add_blinker_vertical(cx, top_base_y as usize);
            }

            let bottom_base_y = cy_i + gap_y - 1;
            if bottom_base_y >= 0 && bottom_base_y + 2 < HEIGHT as isize {
                self.add_blinker_vertical(cx, bottom_base_y as usize);
            }

            let left_base_x = cx_i - gap_x - 1;
            if left_base_x >= 0 && left_base_x + 2 < WIDTH as isize {
                self.add_blinker(left_base_x as usize, (cy_i - 1) as usize);
            }

            let right_base_x = cx_i + gap_x - 1;
            if right_base_x >= 0 && right_base_x + 2 < WIDTH as isize {
                self.add_blinker(right_base_x as usize, (cy_i - 1) as usize);
            }
        }

        // Blinkers colocados a los lados de los pulsares que se pusieron en cruz
        let edge_margin = 8usize;
        self.add_blinker(center_x - 1, edge_margin);                      
        self.add_blinker(center_x - 1, HEIGHT - edge_margin - 1);          
        self.add_blinker(edge_margin, center_y - 1);                       
        self.add_blinker(WIDTH - edge_margin - 3, center_y - 1);

        // Gliders
        self.add_glider(8, center_y - 15);
        self.add_glider_rotated(WIDTH - 8, center_y - 15, 1);
    }
    
    // Pulsar
    fn add_pulsar(&mut self, cx: usize, cy: usize) {
        let pattern = [
            (-4, -6), (-3, -6), (-2, -6), (2, -6), (3, -6), (4, -6),
            (-6, -4), (-1, -4), (1, -4), (6, -4),
            (-6, -3), (-1, -3), (1, -3), (6, -3),
            (-6, -2), (-1, -2), (1, -2), (6, -2),
            (-4, -1), (-3, -1), (-2, -1), (2, -1), (3, -1), (4, -1),
            (-4,  1), (-3,  1), (-2,  1), (2,  1), (3,  1), (4,  1),
            (-6,  2), (-1,  2), (1,  2), (6,  2),
            (-6,  3), (-1,  3), (1,  3), (6,  3),
            (-6,  4), (-1,  4), (1,  4), (6,  4),
            (-4,  6), (-3,  6), (-2,  6), (2,  6), (3,  6), (4,  6),
        ];
        for (dx, dy) in pattern {
            let x = (cx as i32 + dx) as usize;
            let y = (cy as i32 + dy) as usize;
            if x < WIDTH && y < HEIGHT {
                self.grid[y][x] = true;
            }
        }
    }
    
    // Beehive
    fn add_beehive(&mut self, cx: usize, cy: usize) {
        let pattern = [
            (1,0),(2,0),
            (0,1),(3,1),
            (1,2),(2,2)
        ];
        for (dx, dy) in pattern {
            let x = cx + dx; let y = cy + dy;
            if x < WIDTH && y < HEIGHT { self.grid[y][x] = true; }
        }
    }
    
    // Tub
    fn add_tub(&mut self, cx: usize, cy: usize) {
        let pattern = [
            (1,0),
            (0,1),(2,1),
            (1,2)
        ];
        for (dx, dy) in pattern {
            let x = cx + dx; let y = cy + dy;
            if x < WIDTH && y < HEIGHT { self.grid[y][x] = true; }
        }
    }
    
    // Blinker
    fn add_blinker(&mut self, cx: usize, cy: usize) {
        let pattern = [(0,0),(1,0),(2,0)];
        for (dx, dy) in pattern {
            let x = cx + dx; let y = cy + dy;
            if x < WIDTH && y < HEIGHT { self.grid[y][x] = true; }
        }
    }

    // Blinker vertical
    fn add_blinker_vertical(&mut self, cx: usize, cy: usize) {
        let pattern = [(0,0),(0,1),(0,2)];
        for (dx, dy) in pattern {
            let x = cx + dx; let y = cy + dy;
            if x < WIDTH && y < HEIGHT { self.grid[y][x] = true; }
        }
    }
    
    // Añadir un Glider
    fn add_glider(&mut self, cx: usize, cy: usize) {
        let pattern = [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
        for (dx, dy) in pattern {
            let x = cx + dx;
            let y = cy + dy;
            if x < WIDTH && y < HEIGHT {
                self.grid[y][x] = true;
            }
        }
    }
    
    // Añadir un Glider rotado 
    fn add_glider_rotated(&mut self, cx: usize, cy: usize, rotation: u8) {
        let patterns = [
            [(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)],
            [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 0), (2, 0), (0, 1), (1, 2)],
            [(0, 0), (0, 1), (0, 2), (1, 0), (2, 1)],
        ];
        let pattern = patterns[rotation as usize % 4];
        for (dx, dy) in pattern {
            let x = cx + dx;
            let y = cy + dy;
            if x < WIDTH && y < HEIGHT {
                self.grid[y][x] = true;
            }
        }
    }
    
    // Toad
    fn add_toad(&mut self, cx: usize, cy: usize) {
        let pattern = [
            (1,0),(2,0),(3,0),
            (0,1),(1,1),(2,1)
        ];
        for (dx, dy) in pattern {
            let x = cx + dx; let y = cy + dy;
            if x < WIDTH && y < HEIGHT { self.grid[y][x] = true; }
        }
    }
    
    //  Ayuda a actualizar el estado del juego
    pub fn update(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let mut vivos = 0;
                for dy in [-1i32,0,1] {
                    for dx in [-1i32,0,1] {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = (x as i32 + dx + WIDTH as i32) % WIDTH as i32;
                        let ny = (y as i32 + dy + HEIGHT as i32) % HEIGHT as i32;
                        if self.grid[ny as usize][nx as usize] { vivos += 1; }
                    }
                }
                self.next[y][x] = if self.grid[y][x] {
                    vivos == 2 || vivos == 3
                } else {
                    vivos == 3
                };
            }
        }
        self.grid = self.next;
    }

    pub fn render(&self, fb: &mut Framebuffer) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let alive = self.grid[y][x];
                let color = fb.get_color(alive);
                fb.set_pixel(x as i32, y as i32, color);
            }
        }
    }
}
