use std::io::Cursor;
use std::io::Write;
use brotli::CompressorWriter;
use graphics_data::{GraphicsData};
use bincode::serialize;
use chrono::{DateTime, FixedOffset};
use time::Duration;
use std::ops::Sub;
use js_utils::{get_date, random};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

impl MoveDirection {
    pub fn opposite(self, other: MoveDirection) -> bool {
        (self == MoveDirection::Up && other == MoveDirection::Down) ||
            (self == MoveDirection::Down && other == MoveDirection::Up) ||
            (self == MoveDirection::Left && other == MoveDirection::Right) ||
            (self == MoveDirection::Right && other == MoveDirection::Left)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum CollisionType {
    Snake,
    Border,
    Apple,
    None,
}

pub struct GameResult {
    pub apples_eaten: u32,
    pub turns_passed: u32,
    pub history: Vec<u8>,
}

type PreviousMove = (Location, MoveDirection);

pub struct SnakeGameLogic {
    width: u32,
    height: u32,
    snake: Vec<PreviousMove>,
    apple: Location,
    last_direction: MoveDirection,
    key_buffer: Vec<MoveDirection>,
    last_frame: DateTime<FixedOffset>,
    duration_between_frames: Duration,
    history: Vec<GameTurn>,
    apples_eaten: u32,
    turns_passed: u32,
    eaten_this_frame: bool
}

#[derive(Serialize, Deserialize)]
struct GameTurn {
    time: DateTime<FixedOffset>,
    snake: Vec<Location>,
    apple: Location,
    next_direction: MoveDirection,
}

impl SnakeGameLogic {
    pub fn new(width: u32, height: u32, frame_rate: u32) -> Self {
        let mut s = SnakeGameLogic {
            width,
            height,
            snake: vec![(Location { x: (width / 2) as i32, y: (height / 2) as i32 }, MoveDirection::Right)],
            apple: Location { x: 0, y: 0 },
            last_direction: MoveDirection::Right,
            key_buffer: Vec::new(),
            duration_between_frames: Duration::milliseconds(((1.0 / frame_rate as f64) * 1000.0) as i64),
            last_frame: get_date(),
            history: Vec::new(),
            apples_eaten: 0,
            turns_passed: 0,
            eaten_this_frame: false
        };
        s.place_new_apple();
        s
    }

    pub fn press_key(&mut self, direction: MoveDirection) {
        self.key_buffer.push(direction);
    }

    pub fn advance(&mut self) -> Result<GraphicsData, GraphicsData> {
        let now = get_date();
        let mut time_diff: Duration = now - self.last_frame;
        if time_diff >= self.duration_between_frames {
            time_diff = time_diff.sub(self.duration_between_frames);
            self.last_frame = now;
            self.last_direction = self.process_key_buffer();
            self.turns_passed += 1;
            self.record_turn();
            self.eaten_this_frame = false;
            let next = self.next_square();
            match self.detect_collision(&next) {
                CollisionType::Apple => {
                    self.eaten_this_frame = true;
                    self.apples_eaten += 1;
                    self.snake.push((next, self.last_direction));
                    self.place_new_apple();
                }
                CollisionType::None => {
                    self.snake.push((next, self.last_direction));
                    self.lob_tail();
                }
                CollisionType::Snake | CollisionType::Border => {
                    return Err(self.draw_screen(1.0));
                }
            }
        }

        let next_frame_progress = time_diff.num_microseconds().unwrap() as f64
            / self.duration_between_frames.num_microseconds().unwrap() as f64;
        Ok(self.draw_screen(next_frame_progress))
    }

    pub fn get_results(&self) -> GameResult {
        let mut history: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        {
            let mut writer = CompressorWriter::new(&mut history,
                                                   4096,
                                                   9,
                                                   22);
            writer.write_all(serialize(&self.history).unwrap().as_ref()).expect("failure while compressing game replay");
        }
        GameResult {
            apples_eaten: self.apples_eaten,
            turns_passed: self.turns_passed,
            history: history.into_inner(),
        }
    }

    fn process_key_buffer(&mut self) -> MoveDirection {
        let mut next: MoveDirection = self.last_direction;
        let mut buffered: Option<MoveDirection> = None;
        let buff_range = 0..self.key_buffer.len();
        for key in self.key_buffer.drain(buff_range) {
            if key.opposite(self.last_direction) {
                buffered = Some(key);
            } else if key != self.last_direction {
                next = key;
            }
        };
        if let Some(key) = buffered {
            if next != self.last_direction {
                self.key_buffer.push(key);
            }
        }
        next
    }

    fn next_square(&self) -> Location {
        let snake_head = &self.snake.last().unwrap().0;
        match self.last_direction {
            MoveDirection::Up => {
                Location {
                    x: snake_head.x,
                    y: snake_head.y - 1,
                }
            }
            MoveDirection::Down => {
                Location {
                    x: snake_head.x,
                    y: snake_head.y + 1,
                }
            }
            MoveDirection::Left => {
                Location {
                    x: snake_head.x - 1,
                    y: snake_head.y,
                }
            }
            MoveDirection::Right => {
                Location {
                    x: snake_head.x + 1,
                    y: snake_head.y,
                }
            }
        }
    }

    fn detect_collision(&self, loc: &Location) -> CollisionType {
        if loc.x < 0 || loc.x >= self.width as i32 || loc.y < 0 || loc.y >= self.height as i32 {
            return CollisionType::Border;
        }
        if *loc == self.apple {
            return CollisionType::Apple;
        }
        for snake_piece in self.snake.iter() {
            if *loc == snake_piece.0 {
                return CollisionType::Snake;
            }
        }
        CollisionType::None
    }

    fn lob_tail(&mut self) {
        self.snake.remove(0);
    }

    fn draw_screen(&self, progress: f64) -> GraphicsData {
        let mut graphics = GraphicsData::new(self.width, self.height);
        graphics.add_pixel(self.apple.x as u32, self.apple.y as u32, String::from("red"));

        for snake_piece in self.snake[0..(self.snake.len()-1)].iter() {
            let snake_piece = &snake_piece.0;
            graphics.add_pixel(snake_piece.x as u32, snake_piece.y as u32, String::from("green"));
        }

        if !self.eaten_this_frame {
            let first_piece = self.snake.first().unwrap();
            let (c1, c2) = match first_piece.1 {
                MoveDirection::Up => {
                    ((0.0, 1.0 - progress), (1.0, 1.0))
                },
                MoveDirection::Down => {
                    ((0.0, progress - 1.0), (1.0, 1.0))
                },
                MoveDirection::Left => {
                    ((1.0 - progress, 0.0), (1.0, 1.0))
                },
                MoveDirection::Right => {
                    ((progress - 1.0, 0.0), (1.0, 1.0))
                },
            };
            let first_loc = &first_piece.0;
            graphics.add_sub_pixel(first_loc.x as u32, first_loc.y as u32, c1, c2, String::from("green"));
        }

        let last_piece = &self.snake.last().unwrap().0;
        graphics.add_pixel(last_piece.x as u32, last_piece.y as u32, String::from("white"));
        let (c1, c2) = match self.last_direction {
            MoveDirection::Up => {
                ((0.0, 1.0 - progress), (1.0, 1.0))
            },
            MoveDirection::Down => {
                ((0.0, progress - 1.0), (1.0, 1.0))
            },
            MoveDirection::Left => {
                ((1.0 - progress, 0.0), (1.0, 1.0))
            },
            MoveDirection::Right => {
                ((progress - 1.0, 0.0), (1.0, 1.0))
            },
        };
        graphics.add_sub_pixel(last_piece.x as u32, last_piece.y as u32, c1, c2, String::from("blue"));
        graphics
    }

    fn record_turn(&mut self) {
        self.history.push(GameTurn {
            time: self.last_frame.clone(),
            snake: self.snake.clone().into_iter().map(|x| x.0).collect(),
            apple: self.apple.clone(),
            next_direction: self.last_direction,
        });
    }

    fn place_new_apple(&mut self) {
        loop {
            let loc = Location {
                x: (random() * self.width as f64) as i32,
                y: (random() * self.height as f64) as i32,
            };
            if let CollisionType::None = self.detect_collision(&loc) {
                self.apple = loc;
                break;
            }
        }
    }
}
