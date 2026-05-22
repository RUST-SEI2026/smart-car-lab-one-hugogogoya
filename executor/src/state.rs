use crate::action::Action;

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct State {
    pub(crate) is_reverse: bool,
    pub(crate) is_fast: bool,
}

impl State {
    pub(crate) fn be_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    pub(crate) fn be_fast(&mut self) {
        self.is_fast = !self.is_fast;
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
        let base_step = if self.is_reverse { -1 } else { 1 };
        let steps = if self.is_fast { 2 } else { 1 };
        // 如果需要“一格一格前进”，则拆分为两个 Action::Forward(step) 其中 step = base_step
        // 但为了简单且符合要求（前进2格），我们可以生成单个 Forward(2 * base_step)
        // 然而 PPT 要求“不能跳跃，只能一格一格前进”，因此需要生成两个 Forward(base_step)
        let mut actions = Vec::new();
        for _ in 0..steps {
            actions.push(Action::Forward(base_step));
        }
        actions
    }

    fn turn_left_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.is_fast {
            let step = if self.is_reverse { -1 } else { 1 };
            actions.push(Action::Forward(step));
        }
        let turn_action = if self.is_reverse {
            Action::TurnRight
        } else {
            Action::TurnLeft
        };
        actions.push(turn_action);
        actions
    }

    fn turn_right_assemble(&self) -> Vec<Action> {
        let mut actions = Vec::new();
        if self.is_fast {
            let step = if self.is_reverse { -1 } else { 1 };
            actions.push(Action::Forward(step));
        }
        let turn_action = if self.is_reverse {
            Action::TurnLeft
        } else {
            Action::TurnRight
        };
        actions.push(turn_action);
        actions
    }
}