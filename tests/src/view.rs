use eired_display::{Cell, Span, View};

#[test]
fn get_line() {
    let view = View::new(3, 3, vec![Cell::default(); 3 * 3]);

    assert_eq!(view.get_line(0).len(), 3);
    assert_eq!(view.get_line(1).len(), 3);
    assert_eq!(view.get_line(2).len(), 3);
    assert_eq!(view.get_line(3), &[]);
}

#[test]
fn construct_with() {
    let view = View::new(5, 5, Span::from("AAAAABBBBBCCCCCDDDDDEEEEE").to_vec());

    assert_eq!(view.get_line(0), Span::from("AAAAA").to_vec().as_slice());
    assert_eq!(view.get_line(1), Span::from("BBBBB").to_vec().as_slice());
    assert_eq!(view.get_line(2), Span::from("CCCCC").to_vec().as_slice());
    assert_eq!(view.get_line(3), Span::from("DDDDD").to_vec().as_slice());
    assert_eq!(view.get_line(4), Span::from("EEEEE").to_vec().as_slice());
}
