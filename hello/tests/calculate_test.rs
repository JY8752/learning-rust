use hello::calculate;
use rstest::rstest;

#[test]
fn test_add() {
    assert_eq!(calculate::add(1, 2), 3);
}

#[rstest]
#[case(1, 2, 3)]
#[case::one_plus_two(1, 2, 3)]
fn parameters_test(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(expected, calculate::add(a, b));
}
