use executor::{Executor, Pose};

mod fast_tests {
    use super::*;

    // 1. 加速状态下 M：前进 2 格
    #[test]
    fn should_return_x_plus_2_given_fast_and_m_facing_e() {
        let pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(pose);
        executor.execute("FM");
        let expected = Pose::new(2, 0, 'E');
        assert_eq!(expected, executor.query());
    }

    // 2. 加速状态下 L：前进 1 格，然后左转
    #[test]
    fn should_return_x_plus_1_and_heading_n_given_fast_and_l_facing_e() {
        let pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(pose);
        executor.execute("FL");
        let expected = Pose::new(1, 0, 'N');
        assert_eq!(expected, executor.query());
    }

    // 3. 加速状态下 R：前进 1 格，然后右转
    #[test]
    fn should_return_x_plus_1_and_heading_s_given_fast_and_r_facing_e() {
        let pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(pose);
        executor.execute("FR");
        let expected = Pose::new(1, 0, 'S');
        assert_eq!(expected, executor.query());
    }

    // 4. 两次 F 取消加速：M 只前进 1 格
    #[test]
    fn should_return_x_plus_1_given_ffm_facing_e() {
        let pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(pose);
        executor.execute("FFM");
        let expected = Pose::new(1, 0, 'E');
        assert_eq!(expected, executor.query());
    }

    // 5. B + F 叠加状态下 M：后退 2 格
    #[test]
    fn should_return_x_minus_2_given_reverse_fast_and_m_facing_e() {
        let pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(pose);
        executor.execute("BFM");
        let expected = Pose::new(-2, 0, 'E');
        assert_eq!(expected, executor.query());
    }

    // 6. B + F 叠加状态下 L：先倒退一格，再右转
    #[test]
    fn should_return_x_minus_1_and_heading_s_given_reverse_fast_and_l_facing_e() {
        let pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(pose);
        executor.execute("BFL");
        let expected = Pose::new(-1, 0, 'S');
        assert_eq!(expected, executor.query());
    }

    // 7. B + F 叠加状态下 R：先倒退一格，再左转
    #[test]
    fn should_return_x_minus_1_and_heading_n_given_reverse_fast_and_r_facing_e() {
        let pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(pose);
        executor.execute("BFR");
        let expected = Pose::new(-1, 0, 'N');
        assert_eq!(expected, executor.query());
    }
}