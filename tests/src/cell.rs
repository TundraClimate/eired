use crossterm::style::Color;
use eired_display::Cell;

#[test]
fn default_cell() {
    let cell = Cell::default();

    assert_eq!(
        cell,
        Cell {
            ch: ' ',
            fg: Color::Reset,
            bg: Color::Reset
        }
    )
}
