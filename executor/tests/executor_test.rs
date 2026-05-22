use executor::{Executor, Pose};

mod move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_1_given_command_is_m_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("M");
        let expected_pose = Pose::new(1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_minus_1_given_command_is_m_and_facing_is_s() {
        let original_pose = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("M");
        let expected_pose = Pose::new(0, -1, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_given_command_is_m_and_facing_is_w() {
        let original_pose = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("M");
        let expected_pose = Pose::new(-1, 0, 'W');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_m_and_facing_is_n() {
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("M");
        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}

mod turn_tests {
    use super::*;

    // 左转
    #[test]
    fn should_return_heading_n_given_command_is_l_and_facing_is_e() {
        let original = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original);
        executor.execute("L");
        let expected = Pose::new(0, 0, 'N');
        assert_eq!(expected, executor.query());
    }

    #[test]
    fn should_return_heading_e_given_command_is_l_and_facing_is_s() {
        let original = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original);
        executor.execute("L");
        let expected = Pose::new(0, 0, 'E');
        assert_eq!(expected, executor.query());
    }

    #[test]
    fn should_return_heading_s_given_command_is_l_and_facing_is_w() {
        let original = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original);
        executor.execute("L");
        let expected = Pose::new(0, 0, 'S');
        assert_eq!(expected, executor.query());
    }

    #[test]
    fn should_return_heading_w_given_command_is_l_and_facing_is_n() {
        let original = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original);
        executor.execute("L");
        let expected = Pose::new(0, 0, 'W');
        assert_eq!(expected, executor.query());
    }

    // 右转
    #[test]
    fn should_return_heading_s_given_command_is_r_and_facing_is_e() {
        let original = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original);
        executor.execute("R");
        let expected = Pose::new(0, 0, 'S');
        assert_eq!(expected, executor.query());
    }

    #[test]
    fn should_return_heading_w_given_command_is_r_and_facing_is_s() {
        let original = Pose::new(0, 0, 'S');
        let mut executor = Executor::with_pose(original);
        executor.execute("R");
        let expected = Pose::new(0, 0, 'W');
        assert_eq!(expected, executor.query());
    }

    #[test]
    fn should_return_heading_n_given_command_is_r_and_facing_is_w() {
        let original = Pose::new(0, 0, 'W');
        let mut executor = Executor::with_pose(original);
        executor.execute("R");
        let expected = Pose::new(0, 0, 'N');
        assert_eq!(expected, executor.query());
    }

    #[test]
    fn should_return_heading_e_given_command_is_r_and_facing_is_n() {
        let original = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original);
        executor.execute("R");
        let expected = Pose::new(0, 0, 'E');
        assert_eq!(expected, executor.query());
    }
}