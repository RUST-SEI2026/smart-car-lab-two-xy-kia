use executor::{Executor, Pose};

mod fast_tests{
    use super::*;

    #[test]
    fn should_return_x_plus_2_given_command_is_fm_and_facing_is_e(){
        let orginal_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(orginal_pose);

        executor.execute("FM");

        let expected_pose = Pose::new(2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_and_facing_n_given_command_is_fl_and_facing_is_e(){
        let orginal_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(orginal_pose);

        executor.execute("FL");

        let expected_pose = Pose::new(1, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_and_facing_s_given_command_is_fr_and_facing_is_e(){
        let orginal_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(orginal_pose);

        executor.execute("FR");

        let expected_pose = Pose::new(1, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_2_and_facing_e_given_command_is_bfm_and_facing_is_e(){
        let orginal_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(orginal_pose);

        executor.execute("BFM");

        let expected_pose = Pose::new(-2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_and_facing_s_given_command_is_bfl_and_facing_is_e(){
        let orginal_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(orginal_pose);

        executor.execute("BFL");

        let expected_pose = Pose::new(-1, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_and_facing_n_given_command_is_bfr_and_facing_is_e(){
        let orginal_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(orginal_pose);

        executor.execute("BFR");

        let expected_pose = Pose::new(-1, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_ffm_and_facing_is_n(){
        let orginal_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(orginal_pose);

        executor.execute("FFM");

        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}