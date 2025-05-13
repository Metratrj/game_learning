use std::io::{self, Write, stdout};

use crossterm::{
    cursor, execute, queue,
    style::{self, Color, ResetColor, SetForegroundColor, Stylize},
};
use rand::Rng;

use crate::{input::read_direction, map::Map, types::Vector};

#[derive(Debug, Clone)]
pub struct Game {
    pub map: Map,
    pub loot: Vec<LootItem>,
    pub log: Vec<String>,
    pub player: Player,
}

#[derive(Debug, Clone)]
pub struct Player {
    pos: Vector<usize>,
    inventory: Vec<LootItem>,
}

impl Player {
    pub fn new(pos: Vector<usize>, inv: Vec<LootItem>) -> Self {
        Self {
            pos,
            inventory: inv,
        }
    }

    pub fn get_pos(&self) -> &Vector<usize> {
        &self.pos
    }

    pub fn set_pos(&mut self, pos: Vector<usize>) {
        self.pos = pos;
    }

    pub fn get_inventory(&self) -> &Vec<LootItem> {
        &self.inventory
    }

    pub fn set_inventory(&mut self, inv: Vec<LootItem>) {
        self.inventory = inv;
    }
}

#[derive(Debug, Clone)]
pub struct LootItem {
    pub pos: Vector<usize>,
    pub symbol: char,
    pub name: String,
}

impl Game {
    pub fn new(map_w: usize, map_h: usize) -> Self {
        let (map, pos) = Map::new(map_w, map_h);
        let mut loot = Vec::new();

        let mut rng = rand::rng();
        for _ in 0..10 {
            let mut tries = 0;
            loop {
                let x = rng.random_range(1..map.width - 1);
                let y = rng.random_range(1..map.height - 1);
                if map.is_walkable(x, y) && (x, y) != (pos.x, pos.y) {
                    loot.push(LootItem {
                        pos: Vector { x, y },
                        symbol: '!',
                        name: "Health Potion".to_string(),
                    });
                    break;
                }
                tries += 1;
                if tries > 100 {
                    break;
                }
            }
        }
        let inventory: Vec<LootItem> = Vec::new();
        let log: Vec<String> = Vec::new();

        let player: Player = Player { pos, inventory };

        Self {
            map,
            player,
            loot,
            log,
        }
    }

    pub fn draw(&self) -> core::result::Result<(), io::Error> {
        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::Hide,
            cursor::MoveTo(0, 0),
            //Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();

        for y in 0..self.map.height {
            for x in 0..self.map.width {
                let tile = self.map.get_tile(x, y);
                let color = match tile {
                    '#' => Color::DarkGrey,
                    '.' => Color::Black,
                    _ => Color::White,
                };
                queue!(
                    stdout,
                    cursor::MoveTo(x as u16, y as u16),
                    SetForegroundColor(color),
                    style::Print(format!("{}", tile))
                )?;
            }
        }

        for item in &self.loot {
            if item.pos.y < self.map.height && item.pos.x < self.map.width {
                queue!(
                    stdout,
                    cursor::MoveTo(item.pos.x as u16, item.pos.y as u16),
                    SetForegroundColor(Color::Yellow),
                    style::PrintStyledContent(format!("{}", &item.symbol).yellow())
                )?;
                // print!("{}", item.symbol);
            }
        }

        queue!(
            stdout,
            cursor::MoveTo(self.player.pos.x as u16, self.player.pos.y as u16),
            SetForegroundColor(Color::Green),
            style::Print("@".green())
        )?;

        queue!(
            stdout,
            cursor::MoveTo(0, self.map.height as u16),
            ResetColor,
            style::Print("Use W/A/S/D to move, Q to quit.")
        )?;

        queue!(
            stdout,
            cursor::MoveRight((self.map.width / 2) as u16),
            style::Print("Inventory: ")
        )?;

        for item in &self.player.inventory {
            queue!(stdout, style::Print(format!("{}, ", &item.name)))?;
        }

        stdout.flush()?;

        Ok(())
    }

    pub fn move_player(&mut self, x: isize, y: isize) {
        let new_x = (self.player.pos.x as isize + x) as usize;
        let new_y = (self.player.pos.y as isize + y) as usize;

        if self.map.is_walkable(new_x, new_y) {
            self.player.pos.x = new_x;
            self.player.pos.y = new_y;

            // Check loot
            self.pick_up_loot_at(new_x, new_y);
        }
    }

    /// Returns the update of this [`Game`].
    #[allow(dead_code)]
    pub fn update(&mut self) -> bool {
        match read_direction() {
            Some((dx, dy)) => {
                let new_x = (self.player.pos.x as isize + dx) as usize;
                let new_y = (self.player.pos.y as isize + dy) as usize;

                if self.map.is_walkable(new_x, new_y) {
                    self.player.pos.x = new_x;
                    self.player.pos.y = new_y;
                }

                true
            }
            None => false, // Quit
        }
    }

    fn pick_up_loot_at(&mut self, x: usize, y: usize) {
        if let Some(index) = self
            .loot
            .iter()
            .position(|item| item.pos.x == x && item.pos.y == y)
        {
            let item = self.loot.remove(index);
            self.log.push(format!("You picked up a {}!", item.name));
            self.player.inventory.push(item);
        }
    }
}
