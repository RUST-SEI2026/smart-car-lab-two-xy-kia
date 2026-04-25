use executor::{Executor, Pose};

mod reverse_tests{
    use super::*;

    #[test]
    fn should_return_x_minus_1_given_status_is_reverse_command_is_m_and_facing_is_e(){
        let orginal_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(orginal_pose);

        executor.execute("BM");

        let expected_pose = Pose::new(-1, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }
}