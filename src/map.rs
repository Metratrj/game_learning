use rand::Rng;

pub struct Map {
    pub width: usize,
    pub height: usize,
    tiles: Vec<char>,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (usize, usize) {
        ((self.x1 + self.x2) / 2, (self.y1 + self.y2) / 2)
    }

    pub fn intersects_with(&self, other: &Rect) -> bool {
        !(self.x2 <= other.x1 || self.x1 >= other.x2 || self.y2 <= other.y1 || self.y1 >= other.y2)
    }
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = vec!['#'; width * height];
        let mut rooms: Vec<Rect> = Vec::new();
        let max_rooms = 30;
        let min_size = 5;
        let max_size = 10;

        let mut rng = rand::rng();

        for _ in 0..max_rooms {
            let w = rng.random_range(min_size..=max_size);
            let h = rng.random_range(min_size..=max_size);
            let x = rng.random_range(1..(width - w - 1));
            let y = rng.random_range(1..(height - h - 1));

            let new_room = Rect::new(x, y, w, h);

            if rooms.iter().all(|other| !new_room.intersects_with(other)) {
                // Inster room
                Self::apply_room_to_map(&mut tiles, width, &new_room);

                // Tunnel to previus room
                if let Some(prev) = rooms.last() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = prev.center();

                    if rng.random_bool(0.5) {
                        Self::apply_horizontal_tunnel(&mut tiles, width, prev_x, new_x, prev_y);
                        Self::apply_vertical_tunnel(&mut tiles, width, prev_y, new_y, prev_x);
                    } else {
                        Self::apply_horizontal_tunnel(&mut tiles, width, prev_y, new_y, prev_x);
                        Self::apply_vertical_tunnel(&mut tiles, width, prev_x, new_x, prev_y);
                    }
                }
                rooms.push(new_room);
            }
        }

        Self {
            width,
            height,
            tiles,
        }
        /* let mut tiles = vec!['.'; width * height];

        // Rand
        for x in 0..width {
            tiles[x] = '#';
            tiles[x + (height - 1) * width] = '#';
        }

        for y in 0..height {
            tiles[y * width] = '#';
            tiles[y * width + (width - 1)] = '#';
        }

        Self {
            width,
            height,
            tiles,
        } */
    }

    pub fn get_tile(&self, x: usize, y: usize) -> char {
        self.tiles[y * self.width + x]
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y) != '#'
    }

    fn apply_room_to_map(tiles: &mut Vec<char>, width: usize, room: &Rect) {
        for y in room.y1..room.y2 {
            for x in room.x1..room.x2 {
                tiles[y * width + x] = '.';
            }
        }
    }

    fn apply_horizontal_tunnel(
        tiles: &mut Vec<char>,
        width: usize,
        x1: usize,
        x2: usize,
        y: usize,
    ) {
        if y >= tiles.len() / width {
            return;
        }

        let start = x1.min(x2);
        let end = x1.max(x2).min(width - 1);

        for x in start..=end {
            let idx = y * width + x;
            if idx < tiles.len() {
                tiles[idx] = '.';
            }
        }
    }

    fn apply_vertical_tunnel(tiles: &mut Vec<char>, width: usize, y1: usize, y2: usize, x: usize) {
        if x >= width {
            return;
        }

        let start = y1.min(y2);
        let end = y1.max(y2).min(tiles.len() / width - 1);

        for y in start..=end {
            let idx = y * width + x;
            if idx < tiles.len() {
                tiles[idx] = '.';
            }
        }
    }
}
