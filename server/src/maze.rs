use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Path,
    Exit,
}

pub struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = vec![vec![Cell::Wall; width]; height];
        let mut maze = Maze { width, height, grid };
        maze.generate();
        maze
    }

    pub fn generate(&mut self) {
        let seed = rand::thread_rng().gen::<u64>();  // 🌱 Génère une seed unique
        let mut rng = StdRng::seed_from_u64(seed); 

        let mut walls = Vec::new();

        let start_x = rng.gen_range(1..self.width / 2) * 2;
        let start_y = rng.gen_range(1..self.height / 2) * 2;
        self.grid[start_y][start_x] = Cell::Path;

        let directions = [(2, 0), (-2, 0), (0, 2), (0, -2)];

        for &(dx, dy) in &directions {
            let nx = start_x as isize + dx;
            let ny = start_y as isize + dy;
            if nx > 0 && nx < self.width as isize - 1 && ny > 0 && ny < self.height as isize - 1 {
                walls.push((start_x, start_y, nx as usize, ny as usize));
            }
        }

        while let Some((x1, y1, x2, y2)) = walls.pop() {
            if x2 > 0 && x2 < self.width - 1 && y2 > 0 && y2 < self.height - 1 && self.grid[y2][x2] == Cell::Wall {
                self.grid[y2][x2] = Cell::Path;
                self.grid[(y1 + y2) / 2][(x1 + x2) / 2] = Cell::Path;

                for &(dx, dy) in &directions {
                    let nx = x2 as isize + dx;
                    let ny = y2 as isize + dy;
                    if nx > 0 && nx < self.width as isize - 1 && ny > 0 && ny < self.height as isize - 1 {
                        walls.push((x2, y2, nx as usize, ny as usize));
                    }
                }

                walls.shuffle(&mut rng);
            }
        }
    }

    pub fn place_exit(&mut self) {
        let mut rng = rand::thread_rng();
        let mut edges = Vec::new();

        for x in (1..self.width - 1).step_by(2) {
            if self.grid[1][x] == Cell::Path {
                edges.push((x, 0));
            }
            if self.grid[self.height - 2][x] == Cell::Path {
                edges.push((x, self.height - 1));
            }
        }
        for y in (1..self.height - 1).step_by(2) {
            if self.grid[y][1] == Cell::Path {
                edges.push((0, y));
            }
            if self.grid[y][self.width - 2] == Cell::Path {
                edges.push((self.width - 1, y));
            }
        }

        if let Some(&(ex, ey)) = edges.choose(&mut rng) {
            self.grid[ey][ex] = Cell::Exit;
        }
    }

    pub fn display(&self, players_positions: &HashMap<String, Vec<(usize, usize)>>) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if let Some((team, _)) = players_positions.iter().find(|(_, positions)| positions.contains(&(x, y))) {
                    print!("{}", team.chars().next().unwrap());
                } else {
                    match cell {
                        Cell::Wall => print!("1"),
                        Cell::Path => print!("0"),
                        Cell::Exit => print!("E"),
                    }
                }
            }
            println!();
        }
    }

    pub fn to_string(&self, players_positions: &HashMap<String, Vec<(usize, usize)>>) -> String {
        self.grid.iter().enumerate()
            .map(|(y, row)| {
                row.iter().enumerate()
                    .map(|(x, &cell)| {
                        if let Some((team, _)) = players_positions.iter().find(|(_, positions)| positions.contains(&(x, y))) {
                            team.chars().next().unwrap()
                        } else {
                            match cell {
                                Cell::Wall => '1',
                                Cell::Path => '0',
                                Cell::Exit => 'E',
                            }
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }    

    pub fn place_players(&self, teams: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<(usize, usize)>> {
        let mut rng = rand::thread_rng();
        let mut positions_map = HashMap::new();
        let mut occupied_positions = Vec::new();
    
        for (team_name, players) in teams {
            let mut team_positions = Vec::new();
            
            for _ in players {
                loop {
                    let x = rng.gen_range(1..self.width - 1);
                    let y = rng.gen_range(1..self.height - 1);
    
                    if self.grid[y][x] == Cell::Path && !occupied_positions.contains(&(x, y)) {
                        occupied_positions.push((x, y));
                        team_positions.push((x, y));
                        break;
                    }
                }
            }
            positions_map.insert(team_name.clone(), team_positions);
        }
    
        positions_map
    }    
    
}
