use std::io::{Write, stdout};

use crossterm::{ExecutableCommand, cursor};

use crate::{input::read_direction, map::Map};

pub struct Game {
    map: Map,
    player_x: usize,
    player_y: usize,
}

impl Game {
    pub fn new(map_w: usize, map_h: usize) -> Self {
        let (map, player_x, player_y) = Map::new(map_w, map_h);
        Self {
            map,
            player_x,
            player_y,
        }
    }

    pub fn draw(&self) {
        let mut stdout = stdout();
        stdout.execute(cursor::MoveTo(0, 0)).unwrap();

        for y in 0..self.map.height {
            stdout.execute(cursor::MoveTo(0, y as u16)).unwrap();
            for x in 0..self.map.width {
                if x == self.player_x && y == self.player_y {
                    print!("@");
                } else {
                    print!("{}", self.map.get_tile(x, y));
                }
            }
        }

        stdout
            .execute(cursor::MoveTo(0, self.map.height as u16 + 1))
            .unwrap();

        println!("Use W/A/S/D to move, Q to quit.");
        stdout.flush().unwrap();
    }

    pub fn move_player(&mut self, x: isize, y: isize) {
        let new_x = (self.player_x as isize + x) as usize;
        let new_y = (self.player_y as isize + y) as usize;

        if self.map.is_walkable(new_x, new_y) {
            self.player_x = new_x;
            self.player_y = new_y;
        }
    }

    /// Returns the update of this [`Game`].
    #[allow(dead_code)]
    pub fn update(&mut self) -> bool {
        match read_direction() {
            Some((dx, dy)) => {
                let new_x = (self.player_x as isize + dx) as usize;
                let new_y = (self.player_y as isize + dy) as usize;

                if self.map.is_walkable(new_x, new_y) {
                    self.player_x = new_x;
                    self.player_y = new_y;
                }

                true
            }
            None => false, // Quit
        }
    }
}
