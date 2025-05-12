use rand::Rng;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<char>,
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
    pub fn new(width: usize, height: usize) -> (Self, usize, usize) {
        let mut tiles = vec!['#'; width * height];
        let mut rooms: Vec<Rect> = Vec::new();
        let max_rooms = 30;
        let min_size = 5;
        let max_size = 10;

        let mut rng = rand::rng();
        let mut prev_center: Option<(usize, usize)> = None;

        for _ in 0..max_rooms {
            let w = rng.random_range(min_size..=max_size);
            let h = rng.random_range(min_size..=max_size);

            if width <= w + 2 || height <= h + 2 {
                continue; // Raum passt überhaupt nicht rein
            }

            let x = rng.random_range(1..(width - w - 1));
            let y = rng.random_range(1..(height - h - 1));

            let new_room = Rect::new(x, y, w, h);

            // Kein Overlap?
            if rooms.iter().all(|other| !new_room.intersects_with(other)) {
                // Inster room
                Self::apply_room_to_map(&mut tiles, width, &new_room);

                let (new_x, new_y) = new_room.center();

                // Verbindung zum vorherigen Raum
                if let Some((prev_x, prev_y)) = prev_center {
                    if rng.random_bool(0.5) {
                        Self::apply_horizontal_tunnel(&mut tiles, width, prev_x, new_x, prev_y);
                        Self::apply_vertical_tunnel(&mut tiles, width, prev_y, new_y, new_x);
                    } else {
                        Self::apply_vertical_tunnel(&mut tiles, width, prev_y, new_y, prev_x);
                        Self::apply_horizontal_tunnel(&mut tiles, width, prev_x, new_x, new_y);
                    }
                }

                // Verbindung zum nächstgelegenen Raum statt nur zum letzten
                /* if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let mut closest = &rooms[0];
                    let mut closest_dist = distance(new_x, new_y, closest.center());

                    for other in &rooms {
                        let (ox, oy) = other.center();
                        let dist = distance(new_x, new_y, (ox, oy));
                        if dist < closest_dist {
                            closest = other;
                            closest_dist = dist;
                        }
                    }

                    let (cx, cy) = closest.center();

                    if rng.random_bool(0.5) {
                        Self::apply_horizontal_tunnel(&mut tiles, width, cx, new_x, cy);
                        Self::apply_vertical_tunnel(&mut tiles, width, cy, new_y, new_x);
                    } else {
                        Self::apply_horizontal_tunnel(&mut tiles, width, cy, new_y, cx);
                        Self::apply_vertical_tunnel(&mut tiles, width, cx, new_x, new_y);
                    }
                }
                 */

                rooms.push(new_room);
                prev_center = Some((new_x, new_y)); // Nur wenn Raum erfolgreich eingefügt wurde
            }
        }

        let (start_x, start_y) = rooms[0].center();
        (
            Self {
                width,
                height,
                tiles,
            },
            start_x,
            start_y,
        )
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

    #[allow(dead_code)]
    fn apply_room_to_map_roomidx(tiles: &mut Vec<char>, width: usize, room: &Rect, room_idx: u32) {
        for y in room.y1..room.y2 {
            for x in room.x1..room.x2 {
                tiles[y * width + x] = char::from_digit(room_idx, 100).unwrap();
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

#[allow(dead_code)]
fn distance(x1: usize, y1: usize, (x2, y2): (usize, usize)) -> usize {
    let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };
    dx + dy // Manhattan-Distanz
}
