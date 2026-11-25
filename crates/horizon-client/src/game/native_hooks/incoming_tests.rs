#[test]
fn a_cancelled_key_still_types_the_characters_left() {
    // `axb` with 'x' cancelled: the key went no further, `ab` still types.
    assert_eq!(
        super::screen_text_hooked(&super::chars_to_type("axb", true), |c| c == 'x').as_deref(),
        Some("ab")
    );
    // Backspace, Escape and the like are keys, not text.
    assert_eq!(
        super::screen_text_hooked(&super::chars_to_type("a\u{7f}\u{1b}b", true), |_| false)
            .as_deref(),
        Some("ab")
    );
    // Every character cancelled: the box takes none of them.
    assert_eq!(
        super::screen_text_hooked(&super::chars_to_type("ab", true), |_| true),
        None
    );
    // Nothing typed: nothing to hand over.
    assert_eq!(super::chars_to_type("", true).chars().next(), None);
}
