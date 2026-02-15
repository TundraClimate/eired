use eired_display::View;

#[test]
fn get_line() {
    let view = View::new(3, 3, vec![None; 3 * 3]);

    assert_eq!(view.get_line(0), &[None, None, None]);
    assert_eq!(view.get_line(1), &[None, None, None]);
    assert_eq!(view.get_line(2), &[None, None, None]);
    assert_eq!(view.get_line(3), &[]);
}
