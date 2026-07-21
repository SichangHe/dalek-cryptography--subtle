#![cfg(feature = "core_hint_black_box")]

use subtle::Choice;

#[test]
fn deprecated_feature_keeps_choice_available() {
    assert_eq!(Choice::from(0).unwrap_u8(), 0);
    assert_eq!(Choice::from(1).unwrap_u8(), 1);
}
