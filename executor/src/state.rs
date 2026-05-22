use crate::action::Action;

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct State {
    pub(crate) is_reverse: bool,
}

impl State {
    pub(crate) fn be_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    pub(crate) fn assemble(&self, cmd: char) -> Vec<Action> {
        match cmd {
            'M' => self.move_assemble(),
            'L' => self.turn_left_assemble(),
            'R' => self.turn_right_assemble(),
            _ => Vec::new(),
        }
    }

    fn move_assemble(&self) -> Vec<Action> {
        let direction = if self.is_reverse { -1 } else { 1 };
        vec![Action::Forward(direction)]
    }

    fn turn_left_assemble(&self) -> Vec<Action> {
        let action = if self.is_reverse {
            Action::TurnRight
        } else {
            Action::TurnLeft
        };
        vec![action]
    }

    fn turn_right_assemble(&self) -> Vec<Action> {
        let action = if self.is_reverse {
            Action::TurnLeft
        } else {
            Action::TurnRight
        };
        vec![action]
    }
}