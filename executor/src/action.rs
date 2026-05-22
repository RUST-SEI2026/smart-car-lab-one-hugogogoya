use crate::pose::Pose;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Action {
    Forward(i32),
    TurnLeft,
    TurnRight,
}

impl Action {
    pub(crate) fn perform(&self, pose: &mut Pose) {
        match self {
            Action::Forward(steps) => pose.forward(*steps),
            Action::TurnLeft => pose.turn_left(),
            Action::TurnRight => pose.turn_right(),
        }
    }
}