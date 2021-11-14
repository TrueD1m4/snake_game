use std::collections::LinkedList;

use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone, Debug)]
struct Block {
    x: i32,
    y: i32,
}

struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block{
            x: x + 2,
            y,
        });
        body.push_back(Block{
            x: x + 1,
            y
        });
        body.push_back(Block {
            x,
            y
        });

        Snake {
            direction: Direction::Right,
            body,
            tail: None
        }
    }

    pub fn draw(&self, con: &Context, gr: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, gr);
        }
    }

    pub fn head_pos(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            _  => (),
        }

        let (last_x, last_y) = self.head_pos();
        let new_block = match self.direction {
            Direction::Up => {
                Block {
                    x: last_x,
                    y: last_y -1,
                }
            }
            Direction::Down => {
                Block {
                    x: last_x,
                    y: last_y + 1,
                }
            }
            Direction::Left => {
                Block {
                    x: last_x - 1,
                    y: last_y,
                }
            }
            Direction::Right => {
                Block {
                    x: last_x + 1,
                    y: last_y,
                }
            }
        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn head_direction(&self) -> Direction {
        self.direction
    }
}