#[derive(Clone)]
struct Rules {
    creates: Vec<u32>,
    survives: Vec<u32>,
    initial: u32,
    simulation_type: char,
}

struct cellular_automata {
    cells: Vec<u8>,
    width: u32,
    height: u32,
    depth: u32,

    rules: Rules,
}

impl cellular_automata {
    fn index(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.width + z * self.width * self.height) as usize
    }

    pub fn new(width: u32, height: u32, depth: u32, rules: &Rules) -> cellular_automata {
        let cells:Vec<u8> = vec!(0; (width * height * depth) as usize);

        cellular_automata {
            cells,
            width,
            height,
            depth,
            rules: rules.clone(),
        }
    }
    
    fn updateCell(&self, new_cells: &mut Vec<u8>, x: u32, y: u32, z: u32) {
        let idx = self.index(x, y, z);

        let dx = vec!(0, 1, -1);
        let dy = vec!(0, 1, -1);
        let dz = vec!(0, 1, -1);

        let mut count = 0;
        for i in 0..dx.len() {
            for j in 0..dy.len() {
                for k in 0..dz.len() {
                    if i == 0 && j == 0 && k == 0 {
                        continue;
                    }

                    if self.rules.simulation_type == 'V' {
                        if dx[i] + dy[j] + dz[k] != 1 {
                            continue;
                        }
                    }

                    let nx = x as i32 + dx[i];
                    let ny = y as i32 + dy[j];
                    let nz = z as i32 + dz[k];

                    // out of bound
                    if nx < 0 || nx >= self.width as i32 || ny < 0 || ny >= self.height as i32 || nz < 0 || nz >= self.depth as i32 {
                        continue;
                    }

                    let idx = self.index(nx as u32, ny as u32, nz as u32);

                    if self.cells[idx] >= 1 {
                        count += 1;
                    }
                }
            }
        }

        match self.cells[idx] {
            0 => {
                if self.rules.creates.iter().any(|&i| i == count) {
                    new_cells[idx] = self.rules.initial as u8;
                }
            }
            1 => {
                if !self.rules.survives.iter().any(|&i| i == count) {
                    new_cells[idx] = 0;
                }
            }
            _ => {
                new_cells[idx] -= 1;
            }
        }
    }

    pub fn updateState(&mut self) {
        let mut new_cells = self.cells.clone();


        for z in 0..self.depth {
            for y in 0..self.height {
                for x in 0..self.width {
                    self.updateCell(&mut new_cells, x, y, z);
                }
            }
        }

        self.cells = new_cells;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testUpdateState() {
        let test_width = 10;
        let test_height = 10;
        let test_depth = 10;

        let mut automata = cellular_automata::new(test_width, test_height,test_depth, &Rules {
            creates: vec!(4),
            survives: vec!(4),
            initial: 5,
            simulation_type: 'M',
        });

        let index = |x: u32, y: u32, z: u32| -> usize {
            (x + y * test_width + z * test_width * test_height) as usize
        };

        automata.cells[index(5,5,5)] = 5;
        automata.cells[index(5,5,6)] = 5;
        automata.cells[index(7,5,5)] = 5;
        automata.cells[index(7,5,6)] = 5;

        automata.updateState();

        assert_eq!(automata.cells[index(6,5,5)], 5);
        assert_eq!(automata.cells[index(6,5,6)], 5);
        assert_eq!(automata.cells[index(6,5,7)], 0);
        assert_eq!(automata.cells[index(5,5,5)], 4);
        assert_eq!(automata.cells[index(5,5,6)], 4);
        assert_eq!(automata.cells[index(7,5,5)], 4);
        assert_eq!(automata.cells[index(7,5,6)], 4);

    }
}
