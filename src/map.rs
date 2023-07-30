use rand::prelude::*;

const MAX_DIRECTION: usize = 4;
const ROOM_SIZE: usize = 15;

#[derive(Debug, Default, Clone, Copy)]
pub enum RoomType {
    #[default]
    Room1,
    Room2,
    Room2C,
    Room3,
    Room4,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Room {
    pub kind: RoomType,
    pub(crate) direction: Direction,
    pub angle: i32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

pub struct Map {
    pub width: u32,
    pub height: u32,
    pub room_array: Vec<Vec<Option<Room>>>,
    pub seed: u64,
    pub room_1_amount: i32,
    pub room_2_amount: i32,
    pub room_2c_amount: i32,
    pub room_3_amount: i32,
    pub room_4_amount: i32,
}

impl Map {
    pub fn new(width: u32, height: u32, seed: u64) -> Map {
        Map {
            width,
            height,
            room_array: vec![vec![None; width as usize]; height as usize],
            seed,
            room_1_amount: 0,
            room_2_amount: 0,
            room_2c_amount: 0,
            room_3_amount: 0,
            room_4_amount: 0,
        }
    }

    pub fn generate(&mut self, max_rooms: usize) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);

        // Center of map
        let mut temp_x = self.width as usize / 2;
        let mut temp_y = self.height as usize / 2;
        self.room_array[temp_x][temp_y] = Some(Room::default());

        for _ in 0..=max_rooms {
            let dir = rng.gen_range(0..MAX_DIRECTION);

            match dir {
                0 if temp_x < ROOM_SIZE => {
                    temp_x += 1;
                    self.add_room(temp_x - 1, temp_y, Direction::East);
                }
                1 if temp_x > 0 => {
                    temp_x -= 1;
                    self.add_room(temp_x + 1, temp_y, Direction::West);
                }
                2 if temp_y < ROOM_SIZE => {
                    temp_y += 1;
                    self.add_room(temp_x, temp_y - 1, Direction::South);
                }
                3 if temp_y > 0 => {
                    temp_y -= 1;
                    self.add_room(temp_x, temp_y + 1, Direction::North);
                }
                _ => {}
            }
        }

        self.update_room_types(&mut rng);
    }

    fn add_room(&mut self, x: usize, y: usize, direction: Direction) {
        self.room_array[x][y] = Some(Room::default());

        match direction {
            Direction::East => {
                if self.room_array[x - 1][y].is_some() {
                    self.room_array[x][y].unwrap().direction = Direction::East;
                    self.room_array[x - 1][y].unwrap().direction = Direction::West;
                }
            }
            Direction::West => {
                if self.room_array[x + 1][y].is_some() {
                    self.room_array[x][y].unwrap().direction = Direction::West;
                    self.room_array[x + 1][y].unwrap().direction = Direction::East;
                }
            }
            Direction::South => {
                if self.room_array[x][y - 1].is_some() {
                    self.room_array[x][y].unwrap().direction = Direction::South;
                    self.room_array[x][y - 1].unwrap().direction = Direction::North;
                }
            }
            Direction::North => {
                if self.room_array[x][y + 1].is_some() {
                    self.room_array[x][y].unwrap().direction = Direction::North;
                    self.room_array[x][y + 1].unwrap().direction = Direction::South;
                }
            }
        }
    }

    fn update_room_types<R>(&mut self, rng: &mut R)
    where
        R: Rng + ?Sized,
    {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(mut room) = self.room_array[x as usize][y as usize] {
                    let north = y > 0 && self.room_array[x as usize][(y - 1) as usize].is_some();
                    let south = y < self.height - 1
                        && self.room_array[x as usize][(y + 1) as usize].is_some();
                    let east = x < self.width - 1
                        && self.room_array[(x + 1) as usize][y as usize].is_some();
                    let west = x > 0 && self.room_array[(x - 1) as usize][y as usize].is_some();

                    room.kind = match (north, south, east, west) {
                        (true, true, true, true) => {
                            let av_angle = [0, 90, 180, 270];
                            room.angle = *av_angle.choose(rng).unwrap();
                            RoomType::Room4
                        }
                        (true, false, true, false) => {
                            room.angle = 90;
                            RoomType::Room3
                        }
                        (false, true, false, true) => {
                            room.angle = 270;
                            RoomType::Room3
                        }
                        (true, false, false, true) => {
                            let av_angle = [0, 180];
                            room.angle = *av_angle.choose(rng).unwrap();
                            RoomType::Room2
                        }
                        (false, true, true, false) => {
                            let av_angle = [90, 270];
                            room.angle = *av_angle.choose(rng).unwrap();
                            RoomType::Room2
                        }
                        (true, false, true, true) => {
                            room.angle = 0;
                            RoomType::Room2C
                        }
                        (false, true, true, true) => {
                            room.angle = 180;
                            RoomType::Room2C
                        }
                        (true, false, false, false) => {
                            room.angle = 0;
                            RoomType::Room1
                        }
                        (false, true, false, false) => {
                            room.angle = 180;
                            RoomType::Room1
                        }
                        (false, false, true, false) => {
                            room.angle = 90;
                            RoomType::Room1
                        }
                        (false, false, false, true) => {
                            room.angle = 270;
                            RoomType::Room1
                        }
                        _ => room.kind,
                    };

                    self.room_array[x as usize][y as usize] = Some(room);

                    // Update room amounts
                    match room.kind {
                        RoomType::Room1 => self.room_1_amount += 1,
                        RoomType::Room2 => self.room_2_amount += 1,
                        RoomType::Room2C => self.room_2c_amount += 1,
                        RoomType::Room3 => self.room_3_amount += 1,
                        RoomType::Room4 => self.room_4_amount += 1,
                    }
                }
            }
        }
    }
}
