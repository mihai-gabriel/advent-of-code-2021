pub enum Directions {
    Up(i32),
    Down(i32),
    Forward(i32),
}

pub struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    pub fn new() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn get_depth(&self) -> i32 {
        self.depth
    }

    pub fn get_horizontal(&self) -> i32 {
        self.horizontal
    }

    pub fn get_aim(&self) -> i32 {
        self.aim
    }

    pub fn add_depth(&mut self, depth: i32) {
        self.depth += depth;
    }

    pub fn add_horizontal(&mut self, horizontal: i32) {
        self.horizontal += horizontal;
    }

    pub fn add_aim(&mut self, aim: i32) {
        self.aim += aim;
    }

    pub fn final_position(&self) -> i32 {
        self.horizontal * self.depth
    }
}
