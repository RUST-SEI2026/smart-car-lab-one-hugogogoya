#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char, // 'E', 'S', 'W', 'N'
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }

    pub fn forward(&mut self, steps: i32) {
        match self.heading {
            'E' => self.x += steps,
            'S' => self.y -= steps,
            'W' => self.x -= steps,
            'N' => self.y += steps,
            _ => (),
        }
    }

    pub fn turn_left(&mut self) {
        match self.heading {
            'E' => self.heading = 'N',
            'S' => self.heading = 'E',
            'W' => self.heading = 'S',
            'N' => self.heading = 'W',
            _ => (),
        }
    }

    pub fn turn_right(&mut self) {
        match self.heading {
            'E' => self.heading = 'S',
            'S' => self.heading = 'W',
            'W' => self.heading = 'N',
            'N' => self.heading = 'E',
            _ => (),
        }
    }
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}