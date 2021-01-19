mod basic_fixture;
mod failure_messages;
mod inline_snap_test;
mod support;

use failure_messages::my_function;
use k9_released::*;

#[test]
fn smoke_test() {
    assert_equal!(1, 1);

    assert_err!(Err("Error!"));

    assert_greater_than!(2, 1);

    assert_greater_than_or_equal!(1, 1);

    assert_lesser_than!(1, 2);

    assert_lesser_than_or_equal!(1, 1);

    assert_matches_regex!("abc", "abc");

    assert_ok!(Ok(2));

    snapshot!(format!("{:?}", Some(true)), "Some(true)");
}

use k9_released::snapshot;

#[test]
fn rust_nyc_test() {
    snapshot!(
        my_function(),
        r#"
{
    2: "aa",
    3: "aaa",
    4: "aaaa",
    5: "aaaaa",
    6: "aaaaaa",
}
"#
    );
}
