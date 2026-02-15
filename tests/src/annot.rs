use eired_display::Annot;

#[test]
fn get_inner_apex() {
    let annot = Annot::new((0, 0), 50, 50, "");

    assert_eq!(annot.inner_apex_pos(), (49, 49));
}

#[test]
fn get_outer_apex() {
    let annot = Annot::new((0, 0), 50, 50, "");

    assert_eq!(annot.outer_apex_pos(), (50, 50));
}

#[test]
fn conflict_check() {
    let annot1 = Annot::new((0, 0), 4, 1, "");
    let annot2 = Annot::new((3, 0), 4, 1, "");
    let annot3 = Annot::new((0, 0), 1, 4, "");
    let annot4 = Annot::new((0, 3), 1, 4, "");
    let annot5 = Annot::new((4, 0), 4, 1, "");
    let annot6 = Annot::new((0, 4), 1, 4, "");
    let annot7 = Annot::new((2, 0), 2, 1, "");
    let annot8 = Annot::new((0, 0), 6, 1, "");
    let annot9 = Annot::new((0, 0), 6, 1, "");
    let anno10 = Annot::new((0, 1), 6, 1, "");
    let anno11 = Annot::new((0, 0), 1, 6, "");
    let anno12 = Annot::new((1, 0), 1, 6, "");

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
    let annot = Annot::new((0, 0), 50, 50, "");

    assert!(annot.contains_pos(49, 49));
    assert!(!annot.contains_pos(50, 50));
}
