use eired_display::{Annotate, Rect};

#[test]
fn get_inner_apex() {
    let annot = Rect(50, 50).annotate((0, 0));

    assert_eq!(annot.inner_apex_pos(), (49, 49));
}

#[test]
fn get_outer_apex() {
    let annot = Rect(50, 50).annotate((0, 0));

    assert_eq!(annot.outer_apex_pos(), (50, 50));
}

#[test]
fn conflict_check() {
    let annot1 = Rect(4, 1).annotate((0, 0));
    let annot2 = Rect(4, 1).annotate((3, 0));
    let annot3 = Rect(1, 4).annotate((0, 0));
    let annot4 = Rect(1, 4).annotate((0, 3));
    let annot5 = Rect(4, 1).annotate((4, 0));
    let annot6 = Rect(1, 4).annotate((0, 4));
    let annot7 = Rect(2, 1).annotate((2, 0));
    let annot8 = Rect(6, 1).annotate((0, 0));
    let annot9 = Rect(6, 1).annotate((0, 0));
    let anno10 = Rect(6, 1).annotate((0, 1));
    let anno11 = Rect(1, 6).annotate((0, 0));
    let anno12 = Rect(1, 6).annotate((1, 0));

    assert!(annot1.is_conflict(&annot2));
    assert!(annot3.is_conflict(&annot4));
    assert!(!annot1.is_conflict(&annot5));
    assert!(!annot3.is_conflict(&annot6));

    assert!(annot2.is_conflict(&annot1));
    assert!(annot4.is_conflict(&annot3));
    assert!(!annot5.is_conflict(&annot1));
    assert!(!annot6.is_conflict(&annot3));

    assert!(annot7.is_conflict(&annot8));
    assert!(annot8.is_conflict(&annot7));

    assert!(!annot9.is_conflict(&anno10));
    assert!(!anno11.is_conflict(&anno12));
}

#[test]
fn contains_pos() {
    let annot = Rect::new(50, 50).annotate((0, 0));

    assert!(annot.contains_pos(49, 49));
    assert!(!annot.contains_pos(50, 50));
}
