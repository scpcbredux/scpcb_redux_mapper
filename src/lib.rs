use rand::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum RoomType {
    #[default]
    Room1,
    Room2,
    Room2C,
    Room3,
    Room4,
}

#[derive(Debug, Clone, Copy)]
pub struct Room {
    pub kind: RoomType,
    pub angle: f32,
    pub linked_turns: [u8; 4],
}

impl Default for Room {
    fn default() -> Self {
        Self {
            kind: Default::default(),
            angle: -1.0,
            linked_turns: Default::default(),
        }
    }
}

pub struct Map {
    pub room_array: Vec<Vec<Room>>,
    pub seed: u64,
    pub room_1_amount: i32,
    pub room_2_amount: i32,
    pub room_2c_amount: i32,
    pub room_3_amount: i32,
    pub room_4_amount: i32,
}

impl Map {
    pub fn new_from_string(seed: &str) -> Self {
        let seed = seed
            .chars()
            .enumerate()
            .fold(0, |temp, (index, c)| temp ^ ((c as u64) << (index % 24)));
        Self::new(seed)
    }

    pub fn new(seed: u64) -> Self {
        Self {
            room_array: vec![vec![Room::default(); 20]; 20],
            seed,
            room_1_amount: 0,
            room_2_amount: 0,
            room_2c_amount: 0,
            room_3_amount: 0,
            room_4_amount: 0,
        }
    }

    pub fn generate(&mut self) {
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.seed);

        let mut y = 19;
        let mut x = rng.gen_range(8..=13);
        let mut gen_dir = 1;
        let mut gen_dist = rng.gen_range(2..=3);

        while y > 0 {
            match gen_dir {
                0 => {
                    for _ in 0..gen_dist {
                        if x < 19 {
                            x += 1;
                            self.room_array[x][y].angle = 0.0;
                        } else {
                            break;
                        }
                    }
                    gen_dir = 1;
                }
                1 => {
                    for _ in 0..gen_dist {
                        if y > 0 {
                            y -= 1;
                            self.room_array[x][y].angle = 0.0;
                        } else {
                            break;
                        }
                    }
                    gen_dir = if x > 13 {
                        2
                    } else if x < 7 {
                        0
                    } else {
                        rng.gen_range(0..=1) * 2
                    };
                }
                2 => {
                    for _ in 0..gen_dist {
                        if x > 0 {
                            x -= 1;
                            self.room_array[x][y].angle = 0.0;
                        } else {
                            break;
                        }
                    }
                    gen_dir = 1;
                }
                _ => {}
            }
            gen_dist = if gen_dir != 1 {
                rng.gen_range(3..=9)
            } else {
                rng.gen_range(2..=3)
            };
            self.room_array[x][y].angle = 1.0;
            // println!("{:#?}", y);
        }

        for x2 in 0..19 {
            for y2 in 0..19 {
                if self.room_array[x2][y2].angle == 0.0 {
                    if y2 < 12 {
                        gen_dist = rng.gen_range(2..=6);
                        if self.room_array[x2 + 1][y2].angle == 0.0
                            && self.room_array[x2 - 1][y2].angle == 0.0
                        {
                            for i in 0..gen_dist {
                                if !(self.room_array[x2 + 1][y2 + i + 2].angle >= 1.0
                                    || self.room_array[x2 - 1][y2 + i + 2].angle >= 1.0)
                                    && (self.room_array[x2 + 1][y2 + i + 1].angle >= 0.0)
                                        == (self.room_array[x2 - 1][y2 + i + 1].angle >= 0.0)
                                    && (self.room_array[x2][y2 + i + 1].angle >= 0.0)
                                        == (self.room_array[x2 - 1][y2 + i + 1].angle >= 0.0)
                                {
                                    self.room_array[x2][y2 + i + 1].angle = 1.0;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    if x2 < 12 {
                        let gen_dist = rng.gen_range(2..=6);
                        if self.room_array[x2][y2 + 1].angle == 0.0
                            && self.room_array[x2][y2 - 1].angle == 0.0
                        {
                            for i in 0..gen_dist {
                                if !(self.room_array[x2 + i + 2][y2 + 1].angle >= 1.0
                                    || self.room_array[x2 + i + 2][y2 - 1].angle >= 1.0)
                                    && (self.room_array[x2 + i + 1][y2 + 1].angle >= 0.0)
                                        == (self.room_array[x2 + i + 1][y2 - 1].angle >= 0.0)
                                    && (self.room_array[x2 + i + 1][y2].angle >= 0.0)
                                        == (self.room_array[x2 + i + 1][y2 - 1].angle >= 0.0)
                                {
                                    self.room_array[x2 + i + 1][y2].angle = 1.0;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    if y2 >= 8 {
                        let gen_dist = rng.gen_range(2..=6);
                        if self.room_array[x2 + 1][y2].angle == 0.0
                            && self.room_array[x2 - 1][y2].angle == 0.0
                        {
                            for i in 0..gen_dist {
                                if !(self.room_array[x2 + 1][y2 - i - 2].angle >= 1.0
                                    || self.room_array[x2 - 1][y2 - i - 2].angle >= 1.0)
                                    && (self.room_array[x2 + 1][y2 - i - 1].angle >= 0.0)
                                        == (self.room_array[x2 - 1][y2 - i - 1].angle >= 0.0)
                                    && (self.room_array[x2][y2 - i - 1].angle >= 0.0)
                                        == (self.room_array[x2 - 1][y2 - i - 1].angle >= 0.0)
                                {
                                    self.room_array[x2][y2 - i - 1].angle = 1.0;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                    if x2 >= 8 {
                        let gen_dist = rng.gen_range(2..=6);
                        if self.room_array[x2][y2 + 1].angle == 0.0
                            && self.room_array[x2][y2 - 1].angle == 0.0
                        {
                            for i in 0..gen_dist {
                                if !(self.room_array[x2 - i - 2][y2 + 1].angle >= 1.0
                                    || self.room_array[x2 - i - 2][y2 - 1].angle >= 1.0)
                                    && (self.room_array[x2 - i - 1][y2 + 1].angle >= 0.0)
                                        == (self.room_array[x2 - i - 1][y2 - 1].angle >= 0.0)
                                    && (self.room_array[x2 - i - 1][y2].angle >= 0.0)
                                        == (self.room_array[x2 - i - 1][y2 - 1].angle >= 0.0)
                                {
                                    self.room_array[x2 - i - 1][y2].angle = 1.0;
                                } else {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        for x in 0..20 {
            for y in 0..20 {
                let mut has_north = false;
                let mut has_south = false;
                let mut has_east = false;
                let mut has_west = false;

                if self.room_array[x][y].angle >= 0.0 {
                    if x > 0 {
                        has_west = self.room_array[x - 1][y].angle > -1.0;
                    }
                    if x < 19 {
                        has_east = self.room_array[x + 1][y].angle > -1.0;
                    }
                    if y > 0 {
                        has_north = self.room_array[x][y - 1].angle > -1.0;
                    }
                    if y < 19 {
                        has_south = self.room_array[x][y + 1].angle > -1.0;
                    }

                    if has_north && has_south {
                        if has_east && has_west {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room4,
                                angle: rng.gen_range(0.0..=3.0),
                                ..Default::default()
                            };
                        } else if has_east && !has_west {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room3,
                                angle: 3.0,
                                ..Default::default()
                            };
                        } else if !has_east && has_west {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room3,
                                angle: 1.0,
                                ..Default::default()
                            };
                        } else {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room2,
                                angle: rng.gen_range(0.0..=1.0) * 2.0,
                                ..Default::default()
                            };
                        }
                    } else if has_east && has_west {
                        if has_north && !has_south {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room3,
                                angle: 0.0,
                                ..Default::default()
                            };
                        } else if !has_north && has_south {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room3,
                                angle: 2.0,
                                ..Default::default()
                            };
                        } else {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room2,
                                angle: rng.gen_range(0.0..=1.0) * 2.0 + 1.0,
                                ..Default::default()
                            };
                        }
                    } else if has_north {
                        if has_east {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room2C,
                                angle: 0.0,
                                ..Default::default()
                            };
                        } else if has_west {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room2C,
                                angle: 1.0,
                                ..Default::default()
                            };
                        } else {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room1,
                                angle: 0.0,
                                ..Default::default()
                            };
                        }
                    } else if has_south {
                        if has_east {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room2C,
                                angle: 3.0,
                                ..Default::default()
                            };
                        } else if has_west {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room2C,
                                angle: 2.0,
                                ..Default::default()
                            };
                        } else {
                            self.room_array[x][y] = Room {
                                kind: RoomType::Room1,
                                angle: 2.0,
                                ..Default::default()
                            };
                        }
                    } else if has_east {
                        self.room_array[x][y] = Room {
                            kind: RoomType::Room1,
                            angle: 3.0,
                            ..Default::default()
                        };
                    } else {
                        self.room_array[x][y] = Room {
                            kind: RoomType::Room1,
                            angle: 1.0,
                            ..Default::default()
                        };
                    }

                    match self.room_array[x][y].kind {
                        RoomType::Room1 => self.room_1_amount += 1,
                        RoomType::Room2 => self.room_2_amount += 1,
                        RoomType::Room2C => self.room_2c_amount += 1,
                        RoomType::Room3 => self.room_3_amount += 1,
                        RoomType::Room4 => self.room_4_amount += 1,
                    }
                }
            }
        }

        for x in 0..20 {
            for y in 0..20 {
                if self.room_array[x][y].angle > -1.0
                    && self.room_array[x][y].kind != RoomType::Room1
                    && self.room_array[x][y].kind != RoomType::Room2
                {
                    let mut dist = 0;
                    let mut ix = 1;
                    while x + ix < 20 && self.room_array[x + ix][y].angle > -1.0 {
                        ix += 1;
                        if self.room_array[x + ix][y].kind != RoomType::Room1
                            && self.room_array[x + ix][y].kind != RoomType::Room2
                        {
                            dist = ix;
                            break;
                        }
                    }
                    if dist > 0 {
                        self.room_array[x][y].linked_turns[0] = dist as u8;
                        // println!("000");
                    }
                    dist = 0;
                    ix = 1;
                    while x >= ix && self.room_array[x - ix][y].angle > -1.0 {
                        if self.room_array[x - ix][y].kind != RoomType::Room1
                            && self.room_array[x - ix][y].kind != RoomType::Room2
                        {
                            dist = ix;
                            break;
                        }
                        ix += 1;
                    }
                    if dist > 0 {
                        self.room_array[x][y].linked_turns[2] = dist as u8;
                        // println!("222");
                    }
                    dist = 0;
                    let mut iy = 1;
                    while y > iy {
                        if self.room_array[x][y - iy].kind != RoomType::Room1
                            && self.room_array[x][y - iy].kind != RoomType::Room2
                        {
                            dist = iy;
                            break;
                        }
                        iy += 1;
                    }
                    if dist > 0 {
                        self.room_array[x][y].linked_turns[1] = dist as u8;
                        // println!("111");
                    }
                    dist = 0;
                    iy = 1;
                    while y + iy < 20 && self.room_array[x][y + iy].angle > -1.0 {
                        if self.room_array[x][y + iy].kind != RoomType::Room1
                            && self.room_array[x][y + iy].kind != RoomType::Room2
                        {
                            dist = iy;
                            break;
                        }
                        iy += 1;
                    }
                    if dist > 0 {
                        self.room_array[x][y].linked_turns[3] = dist as u8;
                        // println!("333");
                    }
                    // println!("");
                }
            }
        }
    }
}
