use eired_display::{Cell, View};

#[test]
fn get_line() {
    let view = View::new(3, 3, vec![Cell::default(); 3 * 3]);

    assert_eq!(view.get_line(0).len(), 3);
    assert_eq!(view.get_line(1).len(), 3);
    assert_eq!(view.get_line(2).len(), 3);
    assert_eq!(view.get_line(3), &[]);
}
